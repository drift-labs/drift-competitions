use anchor_lang::prelude::*;

use super::constraints::*;
use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;
use anchor_spl::token::{Token, TokenAccount};
use drift::math::safe_math::SafeMath;
use drift::state::spot_market::SpotMarket;
use drift::program::Drift;
use drift::cpi::accounts::RevenuePoolDeposit;

pub fn claim_multiple_entries<'info>(ctx: Context<'_, '_, '_, 'info, ClaimMultipleEntries<'info>>, entries: u64) -> Result<()> {
    let mut competitor = ctx.accounts.competitor.load_mut()?;

    competitor.claim_multiple_entries(entries)?;

    drop(competitor);

    let deposit = entries.safe_div_ceil(100)?; // TODO what formula?

    let cpi_program = ctx.accounts.drift_program.to_account_info().clone();
    let cpi_accounts = RevenuePoolDeposit {
        state: ctx.accounts.drift_state.clone(),
        spot_market: ctx.accounts.spot_market.clone().to_account_info(),
        authority: ctx.accounts.authority.clone().to_account_info(),
        spot_market_vault: ctx.accounts.spot_market_vault.clone().to_account_info(),
        user_token_account: ctx.accounts.user_token_account.clone().to_account_info(),
        token_program: ctx.accounts.token_program.clone().to_account_info(),
    };
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
    drift::cpi::deposit_into_spot_market_revenue_pool(
        cpi_context,
        deposit,
    )?;


    Ok(())
}

#[derive(Accounts)]
pub struct ClaimMultipleEntries<'info> {
    #[account(mut)]
    authority: Signer<'info>,
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
    /// CHECK: checked in cpi
    pub drift_state: AccountInfo<'info>,
    #[account(
        mut,
        constraint = spot_market.load()?.market_index == 0
    )]
    pub spot_market: AccountLoader<'info, SpotMarket>,
    #[account(mut)]
    /// CHECK: checked in cpi
    pub spot_market_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        constraint = &spot_market_vault.mint.eq(&user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub drift_program: Program<'info, Drift>,
}