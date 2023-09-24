use anchor_lang::prelude::*;

use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;
use super::constraints::*;

pub fn settle_competition<'info>(
    _ctx: Context<'_, '_, '_, 'info, SettleCompetition<'info>>,
) -> Result<()> {
    Ok(())
}

#[derive(Accounts)]
pub struct SettleCompetition<'info> {
    #[account(mut)]
    keeper: Signer<'info>,
    #[account(
        mut
    )]
    pub competition: AccountLoader<'info, Competition>,
}
