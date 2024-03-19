use crate::state::Size;
use crate::state::{Competition, CompetitionRoundStatus};
use anchor_lang::prelude::*;

pub fn initialize_competition<'info>(
    ctx: Context<'_, '_, '_, 'info, InitializeCompetition<'info>>,
    params: CompetitionParams,
) -> Result<()> {
    let competition_key = ctx.accounts.competition.key();
    let mut competition = ctx.accounts.competition.load_init()?;

    competition.sponsor_info.sponsor = ctx.accounts.sponsor.key();
    competition.sponsor_info.min_sponsor_amount = params.min_sponsor_amount;
    competition.sponsor_info.max_sponsor_fraction = params.max_sponsor_fraction;

    let (competition_authority, competition_authority_bump) = Pubkey::find_program_address(
        &[b"competition_authority".as_ref(), competition_key.as_ref()],
        ctx.program_id,
    );
    competition.competition_authority = competition_authority;
    competition.competition_authority_bump = competition_authority_bump;

    competition.round_number = 0;

    competition.status = CompetitionRoundStatus::Active;

    competition.next_round_expiry_ts = params.next_round_expiry_ts;
    competition.competition_expiry_ts = params.competition_expiry_ts;
    competition.round_duration = params.round_duration;

    competition.max_entries_per_competitor = params.max_entries_per_competitor;

    competition.number_of_winners = params.number_of_winners;

    Ok(())
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub struct CompetitionParams {
    //scheduling variables
    pub next_round_expiry_ts: i64,
    pub competition_expiry_ts: i64, // when competition ends, perpetual when == 0
    pub round_duration: u64,

    // sponsor details
    pub max_entries_per_competitor: u128,
    pub min_sponsor_amount: u64,
    pub max_sponsor_fraction: u64,

    // number of winners
    pub number_of_winners: u32,
}

#[derive(Accounts)]
pub struct InitializeCompetition<'info> {
    #[account(
        init,
        seeds = [b"competition"],
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
