use anchor_lang::prelude::*;

use super::constraints::*;
use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;

pub fn settle_competitor<'info>(
    ctx: Context<'_, '_, '_, 'info, SettleCompetitor<'info>>,
) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    let mut competitor = ctx.accounts.competitor.load_mut()?;
    let mut competition = ctx.accounts.competition.load_mut()?;
    let user_stats = ctx.accounts.drift_user_stats.load()?;

    competition.settle_competitor(&mut competitor, &user_stats, now)?;

    Ok(())
}

#[derive(Accounts)]
pub struct SettleCompetitor<'info> {
    #[account(mut)]
    keeper: Signer<'info>,
    #[account(mut)]
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
