use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use switchboard_solana::prelude::*;

/// The minimum guess that can be submitted, inclusive.
pub const MIN_RESULT: u32 = 1;
/// The maximum guess that can be submitted, inclusive.
pub const MAX_RESULT: u32 = 10;

pub fn request_randomness<'info>(
    ctx: Context<'_, '_, '_, 'info, RequestRandomness<'info>>,
) -> Result<()> {

    let payer_key = ctx.accounts.payer.key();

    // Create the Switchboard request account.
    let request_init_and_trigger_ctx = FunctionRequestInitAndTrigger {
        request: ctx.accounts.switchboard_request.clone(),
        function: ctx.accounts.switchboard_function.to_account_info(),
        authority: ctx.accounts.payer.to_account_info(),
        function_authority: None,
        escrow: ctx.accounts.switchboard_request_escrow.clone(),
        mint: ctx.accounts.switchboard_mint.to_account_info(),
        state: ctx.accounts.switchboard_state.to_account_info(),
        attestation_queue: ctx.accounts.switchboard_attestation_queue.to_account_info(),
        payer: ctx.accounts.payer.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    };

    let request_params = format!(
        "PID={},MIN_RESULT={},MAX_RESULT={},USER={}",
        crate::id(),
        MIN_RESULT,
        MAX_RESULT,
        payer_key,
    );

    msg!("function {:?}", ctx.accounts.switchboard_function.key());

    request_init_and_trigger_ctx.invoke(
        ctx.accounts.switchboard.clone(),
        // bounty - optional fee to reward oracles for priority processing
        // default: 0 lamports
        Some(1),
        None,
        Some(512),
        Some(request_params.into_bytes()),
        None,
        None
    )?;

    Ok(())
}

#[derive(Accounts)]
pub struct RequestRandomness<'info> {
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
    #[account(
        mut,
        signer,
        owner = system_program.key(),
        constraint = switchboard_request.data_len() == 0 && switchboard_request.lamports() == 0
    )]
    pub switchboard_request: AccountInfo<'info>,
    /// CHECK:
    #[account(
        mut,
        owner = system_program.key(),
        constraint = switchboard_request_escrow.data_len() == 0 && switchboard_request_escrow.lamports() == 0
    )]
    pub switchboard_request_escrow: AccountInfo<'info>,

    // TOKEN ACCOUNTS
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub switchboard_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
}
