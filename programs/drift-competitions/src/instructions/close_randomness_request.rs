use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use switchboard_solana::prelude::*;

pub fn close_randomness_request<'info>(
    ctx: Context<'_, '_, '_, 'info, CloseRandomness<'info>>,
) -> Result<()> {
    // Close the Switchboard request account and its associated token wallet.
    let close_ctx = FunctionRequestClose {
        request: ctx.accounts.switchboard_request.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        escrow: ctx.accounts.switchboard_request_escrow.to_account_info(),
        function: ctx.accounts.switchboard_function.to_account_info(),
        sol_dest: ctx.accounts.payer.to_account_info(),
        escrow_dest: ctx.accounts.escrow_dest.to_account_info(),
        state: ctx.accounts.switchboard_state.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
    };

    close_ctx.invoke(
        ctx.accounts.switchboard.clone(),
    )?;


    Ok(())
}

#[derive(Accounts)]
pub struct CloseRandomness<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    // SWITCHBOARD ACCOUNTS
    /// CHECK: program ID checked.
    #[account(executable, address = SWITCHBOARD_ATTESTATION_PROGRAM_ID)]
    pub switchboard: AccountInfo<'info>,
    /// CHECK:
    #[account(
    seeds = [STATE_SEED],
    seeds::program = switchboard.key(),
    bump = switchboard_state.load()?.bump,
    )]
    pub switchboard_state: AccountLoader<'info, AttestationProgramState>,
    pub switchboard_attestation_queue: AccountLoader<'info, AttestationQueueAccountData>,
    #[account(mut)]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    // The Switchboard Function Request account we will create with a CPI.
    // Should be an empty keypair with no lamports or data.
    /// CHECK:
    #[account(mut)]
    pub switchboard_request: AccountInfo<'info>,
    /// CHECK:
    #[account(
        mut
    )]
    pub switchboard_request_escrow: AccountInfo<'info>,
    /// CHECK:
    #[account(
        mut
    )]
    pub escrow_dest: AccountInfo<'info>,

    // TOKEN ACCOUNTS
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub switchboard_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
}
