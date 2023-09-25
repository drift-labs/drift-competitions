use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;
use drift::state::insurance_fund_stake::InsuranceFundStake;
use drift::state::spot_market::SpotMarket;
use drift::math::constants::QUOTE_SPOT_MARKET_INDEX;

use super::constraints::*;
use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;

pub fn claim_winnings<'info>(_ctx: Context<'_, '_, '_, 'info, ClaimWinnings<'info>>) -> Result<()> {
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
}
