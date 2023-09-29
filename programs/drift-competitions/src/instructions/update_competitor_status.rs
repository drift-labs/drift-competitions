use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions;

use super::constraints::*;
use crate::error::ErrorCode;
use crate::state::{Competition, Competitor, CompetitorStatus};
use drift::state::user::UserStats;
use drift::validate;

pub fn update_competitor_status<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateCompetitorStatus<'info>>,
    new_status: CompetitorStatus,
) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    let mut competitor = ctx.accounts.competitor.load_mut()?;
    let mut competition = ctx.accounts.competition.load_mut()?;

    validate!(
        competitor.status != new_status,
        ErrorCode::CompetitorUpdateInvalid
    )?;
    let user_stats = ctx.accounts.drift_user_stats.load()?;

    competitor.update_status(&mut competition, &user_stats, new_status, now)?;

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateCompetitorStatus<'info> {
    #[account(
        mut,
        constraint = is_sponsor_for_competition(&competition, &sponsor)?,
    )]
    pub competition: AccountLoader<'info, Competition>,
    pub sponsor: Signer<'info>,
    #[account(mut)]
    pub competitor: AccountLoader<'info, Competitor>,
    #[account(
        mut,
        constraint = is_competition_for_competitor(&competitor, &competition)?
    )]
    #[account(
        constraint = is_user_stats_for_competitor(&competitor, &drift_user_stats)?
    )]
    pub drift_user_stats: AccountLoader<'info, UserStats>,
}
