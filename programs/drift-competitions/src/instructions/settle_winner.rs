use anchor_lang::prelude::*;
use anchor_spl::token::TokenAccount;

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
    let vault_balance = ctx.accounts.insurance_fund_vault.amount;

    competition.settle_winner(&mut competitor, &spot_market, vault_balance, now)?;

    if competition.number_of_winners == competition.number_of_winners_settled {
        competition.reset_round(now)?;
    }

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
    #[account(
        constraint = spot_market.load()?.insurance_fund.vault == insurance_fund_vault.key(),
    )]
    pub insurance_fund_vault: Account<'info, TokenAccount>,
}
