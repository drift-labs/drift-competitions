use crate::state::Competition;
use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;

use crate::state::events::CompetitionRoundSummaryRecord;
use drift::math::constants::QUOTE_SPOT_MARKET_INDEX;
use drift::math::insurance::if_shares_to_vault_amount;
use drift::state::spot_market::SpotMarket;

use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;

pub fn receive_randomness(
    ctx: Context<ReceiveRandomness>,
    winner_randomness: u128,
    prize_randomness: u128,
) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    msg!("winner_randomness: {}", winner_randomness);
    msg!("prize_randomness: {}", prize_randomness);

    let mut competition = ctx.accounts.competition.load_mut()?;

    let spot_market = ctx.accounts.spot_market.load()?;
    let vault_balance = ctx.accounts.insurance_fund_vault.amount;

    competition.winner_randomness = winner_randomness;
    competition.prize_randomness = prize_randomness;

    msg!("insurance_fund vault_balance: {}", vault_balance);
    msg!(
        "spot_market.insurance_fund.user_shares: {}",
        spot_market.insurance_fund.user_shares
    );
    msg!(
        "spot_market.insurance_fund.total_shares: {}",
        spot_market.insurance_fund.total_shares
    );
    let (_, prize_odds_numerator, prize_placement) =
        competition.calculate_prize_amount(&spot_market, vault_balance)?;

    competition.resolve_winner_and_prize_randomness(&spot_market, vault_balance)?;

    let prize_value = if_shares_to_vault_amount(
        competition.prize_amount,
        spot_market.insurance_fund.total_shares,
        vault_balance,
    )?;

    let max_prize_bucket_value =
        competition.calculate_sponsor_max_prize(&spot_market, vault_balance)?;

    let competition_key = ctx.accounts.competition.key();

    emit!(CompetitionRoundSummaryRecord {
        competition: competition_key,
        round_number: competition.round_number,
        round_start_ts: competition
            .next_round_expiry_ts
            .safe_sub(competition.round_duration.cast()?)?,
        round_end_ts: competition.next_round_expiry_ts,
        total_score_settled: competition.total_score_settled,

        number_of_winners: competition.number_of_winners,
        number_of_competitors_settled: competition.number_of_competitors_settled,

        prize_amount: competition.prize_amount,
        prize_base: competition.prize_base,
        prize_value,

        prize_placement: prize_placement as u32,
        prize_odds_numerator,
        prize_randomness: competition.prize_randomness,
        prize_randomness_max: competition.prize_randomness_max,

        max_prize_bucket_value,
        insurance_vault_balance: vault_balance,
        protocol_if_shares: spot_market
            .insurance_fund
            .total_shares
            .safe_sub(spot_market.insurance_fund.user_shares)?,
        total_if_shares: spot_market.insurance_fund.total_shares,

        ts: now,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct ReceiveRandomness<'info> {
    // COMPETITION ACCOUNTS
    #[account(mut)]
    pub competition: AccountLoader<'info, Competition>,

    // DRIFT ACCOUNTS
    #[account(
        constraint = spot_market.load()?.market_index == QUOTE_SPOT_MARKET_INDEX,
    )]
    pub spot_market: AccountLoader<'info, SpotMarket>,
    #[account(
        constraint = spot_market.load()?.insurance_fund.vault == insurance_fund_vault.key(),
    )]
    pub insurance_fund_vault: Account<'info, TokenAccount>,

    // SWITCHBOARD ACCOUNTS
    #[account(
        constraint = competition.load()?.switchboard_function == switchboard_function.key()
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        constraint = switchboard_request.validate_signer(
            &switchboard_function,
            &enclave_signer
        )? && competition.load()?.switchboard_function_request == switchboard_request.key()
    )]
    pub switchboard_request: Box<Account<'info, FunctionRequestAccountData>>,
    pub enclave_signer: Signer<'info>,
}
