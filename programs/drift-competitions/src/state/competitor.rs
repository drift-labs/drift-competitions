use crate::state::Size;
use anchor_lang::prelude::*;
use drift::error::DriftResult;
// use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;
use drift::state::user::UserStats;
use static_assertions::const_assert_eq;

#[account(zero_copy)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Competitor {
    pub competition: Pubkey,
    pub competition_round_number: u64,

    pub previous_snapshot_score: u64,
    pub latest_snapshot_score: u64,
    pub bonus_score: u64, // this can be used to claim raffle w/o purchase

    // assign unique range to competitor for random draws
    pub min_draw: u128,
    pub max_draw: u128,
    pub unclaimed_winnings: u64,
    pub unclaimed_winnings_base: u128,
}

impl Size for Competitor {
    const SIZE: usize = 120 + 8;
}

const_assert_eq!(Competitor::SIZE, std::mem::size_of::<Competitor>() + 8);

impl Competitor {
    pub fn calculate_snapshot_score(&self, user_stats: &UserStats) -> DriftResult<u64> {
        // todo: make this a function of the competition
        // then each competition defines how the snapshot score is calculated
        Ok(user_stats.fees.total_fee_paid)
    }

    pub fn calculate_round_score(&self, user_stats: &UserStats) -> DriftResult<u64> {
        let current_snapshot_score = self.calculate_snapshot_score(user_stats)?;

        let round_score = current_snapshot_score
            .safe_sub(self.previous_snapshot_score)?
            .safe_add(self.bonus_score)?;

        Ok(round_score)
    }

    pub fn claim_entry(&mut self) -> DriftResult {
        self.bonus_score = self.bonus_score.saturating_add(1);
        Ok(())
    }
}
