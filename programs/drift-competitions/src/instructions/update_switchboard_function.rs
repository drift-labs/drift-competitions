use crate::signer_seeds::get_function_authority_seeds;
use crate::state::Competition;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token};
use switchboard_solana::prelude::*;

pub const MAX_REQUEST_PARAM_SIZE: u32 = 512;

pub fn update_switchboard_function<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateSwitchboardFunction<'info>>,
) -> Result<()> {
    // Create the Switchboard request account.
    let request_init_ctx = FunctionRequestInit {
        request: ctx.accounts.switchboard_request.clone(),
        function: ctx.accounts.switchboard_function.to_account_info(),
        authority: ctx.accounts.competition_authority.to_account_info(),
        function_authority: None,
        escrow: ctx.accounts.switchboard_request_escrow.clone(),
        mint: ctx.accounts.switchboard_mint.to_account_info(),
        state: ctx.accounts.switchboard_state.to_account_info(),
        attestation_queue: ctx.accounts.switchboard_attestation_queue.to_account_info(),
        payer: ctx.accounts.sponsor.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    };

    let competition_key = ctx.accounts.competition.key();
    let bump = ctx.bumps.get("switchboard_function_authority").unwrap();
    let function_authority_seeds = get_function_authority_seeds(&competition_key, bump);

    request_init_ctx.invoke_signed(
        ctx.accounts.switchboard.clone(),
        Some(MAX_REQUEST_PARAM_SIZE),
        None,
        None,
        &[&function_authority_seeds[..]],
    )?;

    let mut competition = ctx.accounts.competition.load_mut()?;
    competition.switchboard_function = ctx.accounts.switchboard_function.key();
    competition.switchboard_function_request = ctx.accounts.switchboard_request.key();
    competition.switchboard_function_request_escrow = ctx.accounts.switchboard_request_escrow.key();

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateSwitchboardFunction<'info> {
    // COMPETITION ACCOUNTS
    #[account(mut)]
    pub sponsor: Signer<'info>,
    #[account(
        mut,
        constraint = competition.load()?.sponsor_info.sponsor == sponsor.key()
    )]
    pub competition: AccountLoader<'info, Competition>,
    /// CHECK
    #[account(
        constraint = competition.load()?.competition_authority == competition_authority.key()
    )]
    pub competition_authority: AccountInfo<'info>,

    // SWITCHBOARD ACCOUNTS
    #[account(executable, address = SWITCHBOARD_ATTESTATION_PROGRAM_ID)]
    /// CHECK: is switchboard program
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
    #[account(
        mut,
        signer,
        owner = system_program.key(),
        constraint = switchboard_request.data_len() == 0 && switchboard_request.lamports() == 0
    )]
    /// CHECK: checked din cpi
    pub switchboard_request: AccountInfo<'info>,
    #[account(
        mut,
        owner = system_program.key(),
        constraint = switchboard_request_escrow.data_len() == 0 && switchboard_request_escrow.lamports() == 0
    )]
    /// CHECK: checked din cpi
    pub switchboard_request_escrow: AccountInfo<'info>,

    // TOKEN ACCOUNTS
    #[account(address = anchor_spl::token::spl_token::native_mint::ID)]
    pub switchboard_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
}
