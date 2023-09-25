use crate::signer_seeds::get_function_authority_seeds;
use crate::state::Competition;
use anchor_lang::prelude::*;
use anchor_spl::token::Token;
use drift::state::spot_market::{SpotMarket};
use drift::math::constants::QUOTE_SPOT_MARKET_INDEX;
use switchboard_solana::prelude::*;

/// The minimum guess that can be submitted, inclusive.
pub const MIN_RESULT: u32 = 1;
/// The maximum guess that can be submitted, inclusive.
pub const MAX_RESULT: u32 = 1_000_000;

pub fn request_randomness<'info>(
    ctx: Context<'_, '_, '_, 'info, RequestRandomness<'info>>,
) -> Result<()> {
    let competition_key = ctx.accounts.competition.key();
    let function_authority_bump = ctx.accounts.competition.load()?.competition_authority_bump;
    let function_authority_seeds =
        get_function_authority_seeds(&competition_key, &function_authority_bump);

    let request_params = format!(
        "PID={},WINNER_MIN={},WINNER_MAX={},PRIZE_MIN={},PRIZE_MAX={},COMPETITION={}",
        crate::id(),
        MIN_RESULT,
        MAX_RESULT,
        MIN_RESULT,
        MAX_RESULT,
        ctx.accounts.competition.key(),
    );

    let update_request_params = FunctionRequestSetConfig {
        request: ctx.accounts.switchboard_request.clone(),
        authority: ctx.accounts.competition_authority.to_account_info(),
    };

    update_request_params.invoke_signed(
        ctx.accounts.switchboard.clone(),
        request_params.into_bytes(),
        false,
        &[&function_authority_seeds[..]],
    )?;

    // Create the Switchboard request account.
    let request_init_and_trigger_ctx = FunctionRequestTrigger {
        request: ctx.accounts.switchboard_request.clone(),
        function: ctx.accounts.switchboard_function.to_account_info(),
        authority: ctx.accounts.competition_authority.to_account_info(),
        escrow: ctx.accounts.switchboard_request_escrow.clone(),
        state: ctx.accounts.switchboard_state.to_account_info(),
        attestation_queue: ctx.accounts.switchboard_attestation_queue.to_account_info(),
        payer: ctx.accounts.keeper.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
    };

    request_init_and_trigger_ctx.invoke_signed(
        ctx.accounts.switchboard.clone(),
        // bounty - optional fee to reward oracles for priority processing
        // default: 0 lamports
        Some(1),
        None,
        None,
        &[&function_authority_seeds[..]],
    )?;

    let mut competition = ctx.accounts.competition.load_mut()?;

    // todo: need spot market / insurance fund vault balance
    // competition.request_winner_and_prize_draw(&spot_market, vault_balance)?;

    // todo: pass self.total_score_settled / self.prize_randomness_max as params for random request

    Ok(())
}

#[derive(Accounts)]
pub struct RequestRandomness<'info> {
    // COMPETITION ACCOUNTS
    #[account(mut)]
    pub competition: AccountLoader<'info, Competition>,
    #[account(mut)]
    pub keeper: Signer<'info>,
    /// CHECK
    #[account(
        constraint = competition.load()?.competition_authority == competition_authority.key()
    )]
    pub competition_authority: AccountInfo<'info>,

    // DRIFT ACCOUNTS
    #[account(
        constraint = spot_market.load()?.market_index == QUOTE_SPOT_MARKET_INDEX,
    )]
    pub spot_market: AccountLoader<'info, SpotMarket>,
    #[account(
        constraint = spot_market.load()?.insurance_fund.vault == insurance_fund_vault.key(),
    )]
    pub insurance_fund_vault: Account<'info, TokenAccount>,


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
    #[account(
        mut,
        constraint = competition.load()?.switchboard_function == switchboard_function.key()
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        mut,
        constraint = competition.load()?.switchboard_function_request == switchboard_request.key()
    )]
    /// CHECK: cpi checks
    pub switchboard_request: AccountInfo<'info>,
    #[account(
        mut,
        constraint = competition.load()?.switchboard_function_request_escrow == switchboard_request_escrow.key()
    )]
    /// CHECK: cpi checks
    pub switchboard_request_escrow: AccountInfo<'info>,

    // TOKEN ACCOUNTS
    pub token_program: Program<'info, Token>,

    // SYSTEM ACCOUNTS
    pub system_program: Program<'info, System>,
}
