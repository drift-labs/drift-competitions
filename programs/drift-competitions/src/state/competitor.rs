use crate::state::Size;
use crate::utils::apply_rebase_to_competitor_unclaimed_winnings;
use anchor_lang::prelude::*;
use drift::error::DriftResult;
use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;

use drift::math::constants::QUOTE_PRECISION_U64;
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
        // 10 cents of taker volume (entry tier of 10 bps) => 1 ticket
        let taker_fee = user_stats
            .fees
            .total_fee_paid
            .safe_div(QUOTE_PRECISION_U64 / 10000)?;
        Ok(taker_fee)
    }

    pub fn calculate_round_score(&self, user_stats: &UserStats) -> DriftResult<u64> {
        let current_snapshot_score = self.calculate_snapshot_score(user_stats)?;

        let round_score = current_snapshot_score
            .safe_sub(self.previous_snapshot_score)?
            .safe_add(self.bonus_score)?;

        Ok(round_score)
    }

    pub fn claim_entry(&mut self) -> CompetitionResult {
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
        insurance_fund_stake: &mut InsuranceFundStake,
        n_shares: Option<u64>,
    ) -> CompetitionResult {
        validate!(
            spot_market.insurance_fund.shares_base == insurance_fund_stake.if_base,
            ErrorCode::CompetitorNeedsToRebaseInsuranceFundStake
        )?;

        if self.unclaimed_winnings == 0 {
            return Err(ErrorCode::CompetitorHasNoUnclaimedWinnings);
        }

        apply_rebase_to_competitor_unclaimed_winnings(self, spot_market)?;

        // after rebase attempt (so n_shares needs to be rebase conscious)
        let shares_to_claim = match n_shares {
            Some(n_shares) => n_shares,
            None => self.unclaimed_winnings,
        };
        validate!(
            shares_to_claim <= self.unclaimed_winnings,
            ErrorCode::InvalidRoundSettlementDetected,
            "competitor trying to complain too many shares: {} > {}",
            shares_to_claim,
            self.unclaimed_winnings
        )?;

        let old_shares = insurance_fund_stake.checked_if_shares(spot_market)?;

        // settle to competitor's if stake
        // drift::cpi::transfer_admin_if_shares(cpi_context, to_insurance_fund_stake, amount)?;

        // todo: replace with cpi call
        insurance_fund_stake
            .update_if_shares(old_shares.safe_add(shares_to_claim.cast()?)?, spot_market)?;

        let new_shares = insurance_fund_stake.checked_if_shares(spot_market)?;

        validate!(
            old_shares.safe_add(shares_to_claim.cast()?)? == new_shares,
            ErrorCode::InvalidRoundSettlementDetected
        )?;

        self.unclaimed_winnings = self.unclaimed_winnings.safe_sub(shares_to_claim)?;

        validate!(
            old_shares.safe_add(shares_to_claim.cast()?)? == new_shares,
            ErrorCode::InvalidRoundSettlementDetected
        )?;

        Ok(())
    }
}
