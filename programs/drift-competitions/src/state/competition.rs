use crate::state::Size;
use crate::utils::get_random_draw;
use drift::error::DriftResult;

use crate::error::{CompetitionResult, ErrorCode};
use drift::validate;

use anchor_lang::prelude::*;
use drift::state::user::UserStats;
use static_assertions::const_assert_eq;

use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;

use super::Competitor;

#[account(zero_copy)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Competition {
    pub name: [u8; 32],
    pub sponsor: Pubkey,

    pub round_number: u64,

    // entries
    pub number_of_competitors: u128,
    pub max_entries_per_competitor: u128, // set a max entry per competitior
    pub number_of_competitors_settled: u128, //starts at zero and you have to settle everyone to know the winner
    pub total_score_settled: u128, // sum of all scores (when num users settled == num users)

    // scheduling variables
    pub first_round_expiry_ts: i64,
    pub competition_expiry_ts: i64, // when competition ends, perpetual when == 0
    pub round_duration: u64,

    pub winning_draw: u128,
}

impl Size for Competition {
    const SIZE: usize = 176 + 8;
}

const_assert_eq!(Competition::SIZE, std::mem::size_of::<Competition>() + 8);

impl Competition {
    // calculate the end unix timestamp for round_number N
    pub fn calculate_round_end_ts(&self) -> DriftResult<i64> {
        self.first_round_expiry_ts
            .safe_add(self.round_duration.safe_mul(self.round_number)?.cast()?)
    }

    pub fn reset_round(&mut self) -> DriftResult {
        self.total_score_settled = 0;
        self.number_of_competitors_settled = 0;

        Ok(())
    }

    pub fn settle_competitor(
        &mut self,
        competitor: &mut Competitor,
        user_stats: UserStats,
    ) -> DriftResult {
        let round_score = competitor.calculate_score(user_stats)?;

        let new_total_score_settled = self.total_score_settled.safe_add(round_score.cast()?)?;

        competitor.min_draw = self.total_score_settled;
        competitor.max_draw = new_total_score_settled;

        self.total_score_settled = new_total_score_settled;
        self.number_of_competitors_settled = self.number_of_competitors_settled.saturating_add(1);

        Ok(())
    }

    pub fn resolve_round(&mut self) -> CompetitionResult {
        validate!(
            self.number_of_competitors > 0
                && self.number_of_competitors == self.number_of_competitors_settled,
            ErrorCode::CompetitionRoundInSettlementPhase,
            "competition round {:?} is still ongoing",
            self.round_number
        )?;

        self.winning_draw = get_random_draw(self.total_score_settled)?;

        self.reset_round()?;
        Ok(())
    }
}
