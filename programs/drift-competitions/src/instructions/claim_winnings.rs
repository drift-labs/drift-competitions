use anchor_lang::prelude::*;

use super::constraints::*;
use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;

pub fn claim_winnings<'info>(ctx: Context<'_, '_, '_, 'info, ClaimWinnings<'info>>) -> Result<()> {
    let mut competitor = ctx.accounts.competitor.load_mut()?;
    let mut competition = ctx.accounts.competition.load_mut()?;

    // competitor.claim_winnings(&spot_market, insurance_fund_stake)?;

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
}
