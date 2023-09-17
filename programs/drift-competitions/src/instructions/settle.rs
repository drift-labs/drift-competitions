use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;

pub fn settle(_ctx: Context<Settle>, result: u32) -> Result<()> {
    msg!("result {}", result);
    Ok(())
}

#[derive(Accounts)]
pub struct Settle<'info> {
    // RANDOMNESS PROGRAM ACCOUNTS
    /// CHECK: should be payer
    pub payer: AccountInfo<'info>,

    // SWITCHBOARD ACCOUNTS
    pub switchboard_function: AccountLoader<'info, FunctionAccountData>,
    #[account(
        constraint = switchboard_request.validate_signer(
            &switchboard_function.to_account_info(),
            &enclave_signer.to_account_info()
        )?
    )]
    pub switchboard_request: Box<Account<'info, FunctionRequestAccountData>>,
    pub enclave_signer: Signer<'info>,
}
