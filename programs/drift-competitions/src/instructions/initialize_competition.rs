use crate::state::Competition;
use crate::state::Size;
use anchor_lang::prelude::*;

pub fn initialize_competition<'info>(
    ctx: Context<'_, '_, '_, 'info, InitializeCompetition<'info>>,
    params: CompetitionParams,
) -> Result<()> {
    let mut competition = ctx.accounts.competition.load_init()?;

    competition.name = params.name;
    competition.sponsor_info.sponsor = ctx.accounts.sponsor.key();

    competition.round_number = 0;

    competition.next_round_expiry_ts = params.next_round_expiry_ts;
    competition.competition_expiry_ts = params.competition_expiry_ts;
    competition.round_duration = params.round_duration;

    Ok(())
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub struct CompetitionParams {
    pub name: [u8; 32],

    //scheduling variables
    pub next_round_expiry_ts: i64,
    pub competition_expiry_ts: i64, // when competition ends, perpetual when == 0
    pub round_duration: u64,
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
