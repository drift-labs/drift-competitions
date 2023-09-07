use anchor_lang::prelude::*;
use crate::state::Competition;
use crate::state::Size;

pub fn initialize_competition<'info>(
    ctx: Context<'_, '_, '_, 'info, InitializeCompetition<'info>>,
    params: CompetitionParams,
) -> Result<()> {
    let mut competition = ctx.accounts.competition.load_init()?;

    competition.name = params.name;
    competition.sponsor = ctx.accounts.sponsor.key();

    Ok(())
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub struct CompetitionParams {
    pub name: [u8; 32],
}

#[derive(Accounts)]
#[instruction(params: CompetitionParams)]
pub struct InitializeCompetition<'info> {
    #[account(
        init,
        seeds = [b"competition", params.name.as_ref()],
        space = Competition::SIZE,
        bump,
        payer = payer
    )]
    pub competition: AccountLoader<'info, Competition>,
    pub sponsor: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}