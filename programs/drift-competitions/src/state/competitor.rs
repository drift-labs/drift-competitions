use crate::state::Size;
use crate::utils::apply_rebase_to_competitor_unclaimed_winnings;
use anchor_lang::prelude::*;
use drift::error::DriftResult;
use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;

use drift::state::insurance_fund_stake::InsuranceFundStake;
use drift::state::spot_market::SpotMarket;
use drift::state::user::UserStats;
use drift::validate;

use crate::error::{CompetitionResult, ErrorCode};

use drift_macros::assert_no_slop;
use static_assertions::const_assert_eq;

#[assert_no_slop]
#[account(zero_copy)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Competitor {
    pub authority: Pubkey,
    pub competition: Pubkey,
    pub user_stats: Pubkey,

    // assign unique range to competitor for random draws
    pub min_draw: u128,
    pub max_draw: u128,
    pub unclaimed_winnings_base: u128,
    pub unclaimed_winnings: u64,

    pub competition_round_number: u64,

    // score snapshots + bonus
    pub previous_snapshot_score: u64,
    pub latest_snapshot_score: u64,
    pub bonus_score: u64, // this can be used to claim raffle w/o purchase
}

impl Size for Competitor {
    const SIZE: usize = 184 + 8;
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
        // todo: currently enforces only one claim.
        // for more, add economic protection to inspection inspection stack to avoid
        // more than one per transaction
        validate!(
            self.bonus_score == 0,
            ErrorCode::CompetitorHasAlreadyClaimedEntry
        )?;

        self.bonus_score = self.bonus_score.saturating_add(1);

        Ok(())
    }

    pub fn claim_winnings(
        &mut self,
        spot_market: &SpotMarket,
        insurance_fund_stake: InsuranceFundStake,
    ) -> CompetitionResult {
        if self.unclaimed_winnings != 0 {
            apply_rebase_to_competitor_unclaimed_winnings(self, spot_market)?;
        }

        let old_shares = insurance_fund_stake.checked_if_shares(spot_market)?;

        // settle to competitor's if stake
        // drift::cpi::transfer_admin_if_shares(cpi_context, to_insurance_fund_stake, amount)?;

        let new_shares = insurance_fund_stake.checked_if_shares(spot_market)?;

        validate!(
            old_shares.safe_add(self.unclaimed_winnings.cast()?)? == new_shares,
            ErrorCode::InvalidRoundSettlementDetected
        )?;

        self.unclaimed_winnings = 0;

        Ok(())
    }
}
