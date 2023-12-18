use anchor_lang::prelude::*;

use crate::state::{Competition, Competitor};
use drift::math::safe_math::SafeMath;
use drift::state::user::UserStats;

pub fn close_competitor<'info>(
    ctx: Context<'_, '_, '_, 'info, CloseCompetitor<'info>>,
) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    let mut competitor = ctx.accounts.competitor.load_mut()?;
    let mut competition = ctx.accounts.competition.load_mut()?;

    let competitor_user_stats = ctx.accounts.drift_user_stats.load()?;

    competitor.validate_deletion(&competition, &competitor_user_stats, now)?;

    competition.number_of_competitors = competition.number_of_competitors.safe_sub(1)?;

    Ok(())
}

#[derive(Accounts)]
pub struct CloseCompetitor<'info> {
    #[account(
        mut,
        has_one = authority,
        close = authority
    )]
    pub competitor: AccountLoader<'info, Competitor>,
    #[account(mut)]
    pub competition: AccountLoader<'info, Competition>,
    #[account(
        constraint = authority.key.eq(&drift_user_stats.load()?.authority)
    )]
    pub drift_user_stats: AccountLoader<'info, UserStats>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
