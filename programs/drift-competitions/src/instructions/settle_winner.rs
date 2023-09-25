use anchor_lang::prelude::*;

use super::constraints::*;
use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;

pub fn settle_winner<'info>(ctx: Context<'_, '_, '_, 'info, SettleWinner<'info>>) -> Result<()> {
    let mut competitor = ctx.accounts.competitor.load_mut()?;
    let mut competition = ctx.accounts.competition.load_mut()?;

    // todo: add spot_market to this
    // competition.settle_winner(&mut competitor, &spot_market)?;
    Ok(())
}

#[derive(Accounts)]
pub struct SettleWinner<'info> {
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
