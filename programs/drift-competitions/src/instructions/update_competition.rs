use crate::state::Competition;
use anchor_lang::prelude::*;
use super::constraints::is_sponsor_for_competition;

pub fn update_competition<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateCompetition<'info>>,
    params: UpdateCompetitionParams,
) -> Result<()> {
    let mut competition = ctx.accounts.competition.load_init()?;

    if let Some(next_round_expiry_ts) = params.next_round_expiry_ts {
        competition.next_round_expiry_ts = next_round_expiry_ts;
    }

    if let Some(competition_expiry_ts) = params.competition_expiry_ts {
        competition.competition_expiry_ts = competition_expiry_ts;
    }

    if let Some(round_duration) = params.round_duration {
        competition.round_duration = round_duration;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub struct UpdateCompetitionParams {
    pub name: [u8; 32],

    // scheduling variables
    pub next_round_expiry_ts: Option<i64>,
    pub competition_expiry_ts: Option<i64>,
    pub round_duration: Option<u64>,
}

#[derive(Accounts)]
pub struct UpdateCompetition<'info> {
    #[account(
        mut,
        constraint = is_sponsor_for_competition(&competition, &sponsor)?,
    )]
    pub competition: AccountLoader<'info, Competition>,
    pub sponsor: Signer<'info>,
}
