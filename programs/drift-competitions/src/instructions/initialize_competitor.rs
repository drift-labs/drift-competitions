use anchor_lang::prelude::*;

use crate::state::Size;
use crate::state::{Competition, Competitor};
use drift::math::safe_math::SafeMath;
use drift::state::user::UserStats;

pub fn initialize_competitor<'info>(
    ctx: Context<'_, '_, '_, 'info, InitializeCompetitor<'info>>,
) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    let mut competitor = ctx.accounts.competitor.load_init()?;
    let mut competition = ctx.accounts.competition.load_mut()?;

    let competitor_user_stats = ctx.accounts.drift_user_stats.load()?;

    competition.validate_round_is_active(now)?;

    competitor.competition = ctx.accounts.competition.key();
    competitor.competition_round_number = competition.round_number;
    competitor.previous_snapshot_score =
        competitor.calculate_snapshot_score(&competitor_user_stats)?;

    competition.number_of_competitors = competition.number_of_competitors.safe_add(1)?;

    // todo: add competition.bonus_score = 1; ?

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeCompetitor<'info> {
    #[account(
        init,
        seeds = [b"competitor",  competition.key().as_ref(), authority.key().as_ref()],
        space = Competitor::SIZE,
        bump,
        payer = payer
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
