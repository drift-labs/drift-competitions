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

use borsh::{BorshDeserialize, BorshSerialize};
use drift_macros::assert_no_slop;
use static_assertions::const_assert_eq;

use super::Competition;
use crate::state::CompetitionRoundStatus;

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum CompetitorStatus {
    Active,
    Disqualified,
}

impl Default for CompetitorStatus {
    fn default() -> Self {
        CompetitorStatus::Active
    }
}

#[assert_no_slop]
#[account(zero_copy(unsafe))]
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

    pub status: CompetitorStatus,
    pub padding: [u8; 31],
}

impl Size for Competitor {
    const SIZE: usize = 216 + 8;
}

const_assert_eq!(Competitor::SIZE, std::mem::size_of::<Competitor>() + 8);

impl Competitor {
    pub fn update_status(
        &mut self,
        competition: &mut Competition,
        user_stats: &UserStats,
        new_status: CompetitorStatus,
        now: i64,
    ) -> CompetitionResult {
        competition.validate_round_settlement_hasnt_started(now)?;

        if self.status == CompetitorStatus::Active && new_status == CompetitorStatus::Disqualified {
            competition.number_of_competitors = competition.number_of_competitors.safe_sub(1)?;
        } else if self.status == CompetitorStatus::Disqualified
            && new_status == CompetitorStatus::Active
        {
            competition.number_of_competitors = competition.number_of_competitors.safe_add(1)?;
            self.competition_round_number = competition.round_number;
            self.previous_snapshot_score = self.calculate_snapshot_score(&user_stats)?;
            self.bonus_score = 0;
        }

        msg!(
            "updating Competitor status: {:?} -> {:?}",
            self.status,
            new_status
        );
        self.status = new_status;

        Ok(())
    }

    pub fn is_active(&self) -> CompetitionResult<bool> {
        Ok(self.status == CompetitorStatus::Active)
    }

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
        self.bonus_score = self.bonus_score.saturating_add(1);

        Ok(())
    }

    pub fn claim_winnings(
        &mut self,
        competition: &mut Competition,
        spot_market: &SpotMarket,
        insurance_fund_stake: &InsuranceFundStake,
        n_shares: Option<u64>,
        now: i64,
    ) -> CompetitionResult<u64> {
        // cpi update to insurance fund stake occurs outside this (in claim instruction)

        // don't allow claiming during round resolution stages / expired
        validate!(
            competition.status == CompetitionRoundStatus::Active
                || competition.status == CompetitionRoundStatus::WinnerSettlementComplete,
            ErrorCode::CompetitionRoundOngoing
        )?;

        validate!(!competition.is_expired(now)?, ErrorCode::CompetitionExpired)?;

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
            shares_to_claim > 0,
            ErrorCode::CompetitorHasInvalidClaim,
            "competitor trying to claim 0 shares: {} vs {}",
            shares_to_claim,
            self.unclaimed_winnings
        )?;

        validate!(
            shares_to_claim <= self.unclaimed_winnings,
            ErrorCode::CompetitorHasInvalidClaim,
            "competitor trying to claim too many shares: {} > {}",
            shares_to_claim,
            self.unclaimed_winnings
        )?;

        self.unclaimed_winnings = self.unclaimed_winnings.safe_sub(shares_to_claim)?;
        competition.outstanding_unclaimed_winnings = competition
            .outstanding_unclaimed_winnings
            .saturating_sub(shares_to_claim.cast::<u128>()?);

        Ok(shares_to_claim)
    }
}
