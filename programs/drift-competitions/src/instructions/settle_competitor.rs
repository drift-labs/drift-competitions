use anchor_lang::prelude::*;

use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;
use super::constraints::*;

pub fn settle_competitor<'info>(
    _ctx: Context<'_, '_, '_, 'info, SettleCompetitor<'info>>,
) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct SettleCompetitor<'info> {
    keeper: Signer<'info>,
    #[account(
        mut
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
