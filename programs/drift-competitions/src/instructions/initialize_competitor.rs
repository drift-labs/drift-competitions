use anchor_lang::prelude::*;

use crate::error::ErrorCode;
use crate::instructions::constraints::is_user_stats_for_competitor;
use crate::state::Size;
use crate::state::{Competition, CompetitionRoundStatus, Competitor};
use drift::math::safe_math::SafeMath;
use drift::state::user::UserStats;
use drift::validate;

pub fn initialize_competitor<'info>(
    ctx: Context<'_, '_, '_, 'info, InitializeCompetitor<'info>>,
) -> Result<()> {
    let mut competitor = ctx.accounts.competitor.load_init()?;
    let mut competition = ctx.accounts.competition.load_mut()?;

    let competitor_user_stats = ctx.accounts.drift_user_stats.load()?;

    validate!(
        competition.status != CompetitionRoundStatus::Expired,
        ErrorCode::InvalidRoundSettlementDetected,
        "Competition is Expired"
    )?;

    validate!(
        competition.status == CompetitionRoundStatus::Active,
        ErrorCode::InvalidRoundSettlementDetected,
        "Cannot initialize competitior account during competition settlement"
    )?;

    competitor.competition = ctx.accounts.competition.key();
    competitor.competition_round_number = competition.round_number;
    competitor.previous_snapshot_score =
        competitor.calculate_snapshot_score(&competitor_user_stats)?;

    competition.number_of_competitors = competition.number_of_competitors.safe_add(1)?;

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
    pub competition: AccountLoader<'info, Competition>,
    #[account(
        mut,
        constraint = is_user_stats_for_competitor(&competitor, &drift_user_stats)?
    )]
    /// CHECK: checked in drift cpi
    pub drift_user_stats: AccountLoader<'info, UserStats>,
    pub authority: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}
