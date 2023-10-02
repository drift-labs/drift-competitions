use crate::state::Competition;
use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;

use drift::math::constants::QUOTE_SPOT_MARKET_INDEX;
use drift::state::spot_market::SpotMarket;

pub fn receive_randomness(
    ctx: Context<ReceiveRandomness>,
    winner_randomness: u128,
    prize_randomness: u128,
) -> Result<()> {
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

    competition.resolve_winner_and_prize_randomness(&spot_market, vault_balance)?;

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
            &switchboard_function.to_account_info(),
            &enclave_signer.to_account_info()
        )? && competition.load()?.switchboard_function_request == switchboard_request.key()
    )]
    pub switchboard_request: Box<Account<'info, FunctionRequestAccountData>>,
    pub enclave_signer: Signer<'info>,
}
