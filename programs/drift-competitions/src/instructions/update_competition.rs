use super::constraints::is_sponsor_for_competition;
use crate::state::Competition;
use anchor_lang::prelude::*;

pub fn update_competition<'info>(
    ctx: Context<'_, '_, '_, 'info, UpdateCompetition<'info>>,
    params: UpdateCompetitionParams,
) -> Result<()> {
    let mut competition = ctx.accounts.competition.load_mut()?;

    if let Some(next_round_expiry_ts) = params.next_round_expiry_ts {
        competition.next_round_expiry_ts = next_round_expiry_ts;
    }

    if let Some(competition_expiry_ts) = params.competition_expiry_ts {
        competition.competition_expiry_ts = competition_expiry_ts;
    }

    if let Some(round_duration) = params.round_duration {
        competition.round_duration = round_duration;
    }

    if let Some(max_entries_per_competitor) = params.max_entries_per_competitor {
        competition.max_entries_per_competitor = max_entries_per_competitor;
    }

    if let Some(min_sponsor_amount) = params.min_sponsor_amount {
        competition.sponsor_info.min_sponsor_amount = min_sponsor_amount;
    }

    if let Some(max_sponsor_fraction) = params.max_sponsor_fraction {
        competition.sponsor_info.max_sponsor_fraction = max_sponsor_fraction;
    }

    if let Some(number_of_winners) = params.number_of_winners {
        competition.number_of_winners = number_of_winners;
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, AnchorSerialize, AnchorDeserialize, PartialEq, Eq)]
pub struct UpdateCompetitionParams {
    // scheduling variables
    pub next_round_expiry_ts: Option<i64>,
    pub competition_expiry_ts: Option<i64>,
    pub round_duration: Option<u64>,

    // sponsor details
    pub max_entries_per_competitor: Option<u128>,
    pub min_sponsor_amount: Option<u64>,
    pub max_sponsor_fraction: Option<u64>,

    // number of winners
    pub number_of_winners: Option<u32>,
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
