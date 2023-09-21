use crate::state::Competitor;

use anchor_lang::prelude::*;
use drift::state::user::UserStats;

pub fn is_user_stats_for_competitor(
    competitor: &AccountLoader<Competitor>,
    user_stats: &AccountLoader<UserStats>,
) -> anchor_lang::Result<bool> {
    Ok(competitor.load()?.user_stats.eq(&user_stats.key()))
}
