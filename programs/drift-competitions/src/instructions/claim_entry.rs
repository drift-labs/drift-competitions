use anchor_lang::prelude::*;
use anchor_lang::solana_program::sysvar::instructions;

use super::constraints::*;
use crate::error::ErrorCode;
use crate::state::{Competition, Competitor};
use drift::state::user::UserStats;
use std::str::FromStr;

pub fn claim_entry<'info>(ctx: Context<'_, '_, '_, 'info, ClaimEntry<'info>>) -> Result<()> {
    let ixs = ctx.accounts.instructions.as_ref();
    let mut current_index = instructions::load_current_index_checked(ixs)? as usize;

    if instructions::load_instruction_at_checked(current_index + 1, ixs).is_ok() {
        msg!("claim entry must be last ix");
        return Err(ErrorCode::Default.into());
    }

    loop {
        if current_index == 0 {
            break;
        }

        current_index -= 1;

        let ix = instructions::load_instruction_at_checked(current_index, ixs)?;

        let compute_program_id = Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap();
        if ix.program_id != compute_program_id {
            msg!("found ix that is not compute budget {:?}", ix.program_id);
            return Err(ErrorCode::Default.into());
        }
    }

    let mut competitor = ctx.accounts.competitor.load_mut()?;

    competitor.claim_entry()?;

    Ok(())
}

#[derive(Accounts)]
pub struct ClaimEntry<'info> {
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
    /// CHECK: fixed instructions sysvar account
    #[account(address = instructions::ID)]
    pub instructions: UncheckedAccount<'info>,
}
