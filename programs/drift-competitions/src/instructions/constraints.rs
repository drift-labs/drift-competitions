use crate::state::{Competition, Competitor};

use anchor_lang::prelude::*;
use drift::state::user::UserStats;

pub fn can_sign_for_competitor<'info>(
    competitor: &AccountLoader<'info, Competitor>,
    authority: &Signer<'info>,
) -> Result<bool> {
    Ok(competitor.load()?.authority.eq(&authority.key()))
}

pub fn is_user_stats_for_competitor(
    competitor: &AccountLoader<Competitor>,
    user_stats: &AccountLoader<UserStats>,
) -> Result<bool> {
    Ok(competitor.load()?.user_stats.eq(&user_stats.key()))
}

pub fn is_competition_for_competitor(
    competitor: &AccountLoader<Competitor>,
    competition: &AccountLoader<Competition>,
) -> Result<bool> {
    Ok(competitor.load()?.competition.eq(&competition.key()))
}

pub fn is_sponsor_for_competition<'info>(
    competition: &AccountLoader<'info, Competition>,
    sponsor: &Signer<'info>,
) -> Result<bool> {
    Ok(competition.load()?.sponsor_info.sponsor.eq(&sponsor.key()))
}
