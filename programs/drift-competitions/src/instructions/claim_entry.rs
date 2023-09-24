use anchor_lang::prelude::*;

use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;
use super::constraints::*;

pub fn claim_entry<'info>(
    _ctx: Context<'_, '_, '_, 'info, ClaimEntry<'info>>,
) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct ClaimEntry<'info> {
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
