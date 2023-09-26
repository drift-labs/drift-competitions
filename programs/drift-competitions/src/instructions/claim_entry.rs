use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions;

use super::constraints::*;
use crate::error::ErrorCode;
use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;

pub fn claim_entry<'info>(ctx: Context<'_, '_, '_, 'info, ClaimEntry<'info>>) -> Result<()> {
    let ixs = ctx.accounts.instructions.as_ref();
    let current_index = instructions::load_current_index_checked(ixs)? as usize;

    if current_index != 0 {
        msg!("claim entry must be first ix");
        return Err(ErrorCode::Default.into());
    }

    if instructions::load_instruction_at_checked(1, ixs).is_ok() {
        msg!("claim entry must be only ix");
        return Err(ErrorCode::Default.into());
    }

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
    /// CHECK: fixed instructions sysvar account
    #[account(address = instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
}
