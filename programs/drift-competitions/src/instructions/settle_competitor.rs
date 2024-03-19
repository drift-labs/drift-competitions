use anchor_lang::prelude::*;

use super::constraints::*;
use crate::error::ErrorCode;
use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;
use drift::validate;

pub fn settle_competitor<'info>(
    ctx: Context<'_, '_, '_, 'info, SettleCompetitor<'info>>,
) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;

    let competitor_pubkey = ctx.accounts.competitor.key();
    let competition_pubkey = ctx.accounts.competition.key();

    let mut competitor = ctx.accounts.competitor.load_mut()?;
    let mut competition = ctx.accounts.competition.load_mut()?;
    let user_stats = ctx.accounts.drift_user_stats.load()?;

    competition.settle_competitor(
        &mut competitor,
        &user_stats,
        now,
        competitor_pubkey,
        competition_pubkey,
    )?;

    if competition.number_of_competitors == competition.number_of_competitors_settled {
        validate!(
            competition.total_score_settled != 0,
            ErrorCode::InvalidRoundSettlementDetected,
            "total_score_settled is 0 after settling all competitors, round cannot end until competitors have one entry"
        )?;

        competition.update_settlement_complete()?;
    }

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
