use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;
use crate::state::Competition;

pub fn receive_randomness(_ctx: Context<Settle>, winner_randomness: u32, prize_randomness: u32) -> Result<()> {
    msg!("winner_randomness {}", winner_randomness);
    msg!("prize_randomness {}", prize_randomness);

    let mut competition = _ctx.accounts.competition.load_mut()?;
    competition.winner_randomness = winner_randomness;
    competition.prize_randomness = prize_randomness;

    Ok(())
}

#[derive(Accounts)]
pub struct Settle<'info> {
    // RANDOMNESS PROGRAM ACCOUNTS
    #[account(mut)]
    pub competition: AccountLoader<'info, Competition>,

    // SWITCHBOARD ACCOUNTS
    #[account(
        constraint = competition.load()?.switchboard_function == switchboard_function.key()
    )]
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        constraint = switchboard_request.validate_signer(
            &switchboard_function.to_account_info(),
            &enclave_signer.to_account_info()
        )? && competition.load()?.switchboard_function_request == switchboard_request.key()
    )]
    pub switchboard_request: Box<Account<'info, FunctionRequestAccountData>>,
    pub enclave_signer: Signer<'info>,
}
