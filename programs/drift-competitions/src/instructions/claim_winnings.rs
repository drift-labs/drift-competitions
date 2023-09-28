use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use drift::math::constants::QUOTE_SPOT_MARKET_INDEX;
use drift::state::insurance_fund_stake::InsuranceFundStake;
use drift::state::spot_market::SpotMarket;

use super::constraints::*;
use crate::error::ErrorCode;
use crate::signer_seeds::get_competition_authority_seeds;
use crate::state::{Competition, Competitor};
use drift::cpi::accounts::TransferProtocolIfShares;
use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;
use drift::program::Drift;
use drift::state::user::UserStats;
use drift::validate;

pub fn claim_winnings<'info>(
    ctx: Context<'_, '_, '_, 'info, ClaimWinnings<'info>>,
    n_shares: Option<u64>,
) -> Result<()> {
    let competition_key = ctx.accounts.competition.key();
    let bump = ctx.accounts.competition.load()?.competition_authority_bump;
    let competition_authority_seeds = get_competition_authority_seeds(&competition_key, &bump);
    let siger_seeds = &[&competition_authority_seeds[..]];

    let mut competitor = ctx.accounts.competitor.load_mut()?;

    let spot_market = ctx.accounts.spot_market.load()?;
    let insurance_fund_stake = ctx.accounts.insurance_fund_stake.load()?;

    let shares_before = insurance_fund_stake.checked_if_shares(&spot_market)?;
    let shares_to_claim =
        competitor.claim_winnings(&spot_market, &insurance_fund_stake, n_shares)?;

    drop(spot_market);
    drop(insurance_fund_stake);

    let cpi_program = ctx.accounts.drift_program.to_account_info().clone();
    let cpi_accounts = TransferProtocolIfShares {
        signer: ctx.accounts.competition_authority.clone(),
        spot_market: ctx.accounts.spot_market.clone().to_account_info(),
        insurance_fund_stake: ctx.accounts.insurance_fund_stake.clone().to_account_info(),
        insurance_fund_vault: ctx.accounts.insurance_fund_vault.clone().to_account_info(),
        authority: ctx.accounts.authority.clone().to_account_info(),
        user_stats: ctx.accounts.drift_user_stats.clone().to_account_info(),
        state: ctx.accounts.drift_state.clone(),
        transfer_config: ctx.accounts.drift_transfer_config.clone(),
    };
    let cpi_context = CpiContext::new_with_signer(cpi_program, cpi_accounts, siger_seeds);
    drift::cpi::transfer_protocol_if_shares(
        cpi_context,
        QUOTE_SPOT_MARKET_INDEX,
        shares_to_claim.cast::<u128>()?,
    )?;

    let spot_market = ctx.accounts.spot_market.load()?;
    let insurance_fund_stake = ctx.accounts.insurance_fund_stake.load()?;

    let shares_after = insurance_fund_stake.checked_if_shares(&spot_market)?;

    validate!(
        shares_before.safe_add(shares_to_claim.cast()?)? == shares_after,
        ErrorCode::InvalidRoundSettlementDetected
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimWinnings<'info> {
    #[account(mut)]
    authority: Signer<'info>,
    #[account(
        mut,
        constraint = can_sign_for_competitor(&competitor, &authority)?,
    )]
    pub competitor: AccountLoader<'info, Competitor>,
    #[account(
        mut,
        constraint = is_competition_for_competitor(&competitor, &competition)?
    )]
    pub competition: AccountLoader<'info, Competition>,
    #[account(
        constraint = is_user_stats_for_competitor(&competitor, &drift_user_stats)?
    )]
    pub drift_user_stats: AccountLoader<'info, UserStats>,
    #[account(
        mut,
        constraint = spot_market.load()?.market_index == QUOTE_SPOT_MARKET_INDEX,
    )]
    pub spot_market: AccountLoader<'info, SpotMarket>,
    #[account(
        constraint = spot_market.load()?.insurance_fund.vault == insurance_fund_vault.key(),
    )]
    pub insurance_fund_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        constraint = insurance_fund_stake.load()?.authority == authority.key(),
    )]
    pub insurance_fund_stake: AccountLoader<'info, InsuranceFundStake>,
    pub drift_program: Program<'info, Drift>,
    /// CHECK
    #[account(
        constraint = competition.load()?.competition_authority == competition_authority.key()
    )]
    pub competition_authority: AccountInfo<'info>,
    /// CHECK in cpi
    pub drift_transfer_config: AccountInfo<'info>,
    /// CHECK in cpi
    pub drift_state: AccountInfo<'info>,
}
