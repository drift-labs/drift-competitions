use anchor_lang::prelude::*;

use super::constraints::*;
use crate::state::{Competition, Competitor};
use drift::math::constants::QUOTE_SPOT_MARKET_INDEX;
use drift::state::spot_market::SpotMarket;
use drift::state::user::UserStats;

pub fn settle_winner<'info>(ctx: Context<'_, '_, '_, 'info, SettleWinner<'info>>) -> Result<()> {
    let clock = Clock::get()?;
    let now = clock.unix_timestamp;
    
    let mut competitor = ctx.accounts.competitor.load_mut()?;
    let mut competition = ctx.accounts.competition.load_mut()?;
    let spot_market = ctx.accounts.spot_market.load()?;

    competition.settle_winner(&mut competitor, &spot_market, now)?;
    competition.reset_round(now)?;
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
    #[account(
        mut,
        constraint = spot_market.load()?.market_index == QUOTE_SPOT_MARKET_INDEX,
    )]
    pub spot_market: AccountLoader<'info, SpotMarket>,
}
