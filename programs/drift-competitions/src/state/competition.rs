use crate::state::events::CompetitionRoundWinnerRecord;
use crate::state::Size;
use crate::utils::{
    apply_rebase_to_competition_prize, apply_rebase_to_competitor_unclaimed_winnings,
};
use drift::{
    error::DriftResult,
    math::constants::{PERCENTAGE_PRECISION, PERCENTAGE_PRECISION_U64, QUOTE_PRECISION},
    state::spot_market::SpotMarket,
};

use crate::error::{CompetitionResult, ErrorCode};
use drift::validate;

use anchor_lang::prelude::*;
use drift::state::user::UserStats;
use static_assertions::const_assert_eq;

use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;

use super::Competitor;
use borsh::{BorshDeserialize, BorshSerialize};

use drift_macros::assert_no_slop;

use drift::math::insurance::{if_shares_to_vault_amount, vault_amount_to_if_shares};

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum CompetitionRoundStatus {
    Active = 0,
    WinnerAndPrizeRandomnessRequested = 1,
    WinnerAndPrizeRandomnessComplete = 2,
    WinnerSettlementComplete = 3,
    Expired = 4,
}

impl Default for CompetitionRoundStatus {
    fn default() -> Self {
        CompetitionRoundStatus::Active
    }
}

#[zero_copy(unsafe)]
#[derive(Default, Eq, PartialEq, Debug)]
pub struct SponsorInfo {
    pub sponsor: Pubkey,
    pub min_sponsor_amount: u64,   // always leave this amount for sponsor
    pub max_sponsor_fraction: u64, // only take this percent of gain above the min amount
}

#[assert_no_slop]
#[account(zero_copy(unsafe))]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Competition {
    pub name: [u8; 32],
    pub sponsor_info: SponsorInfo,

    pub switchboard_function: Pubkey,
    pub switchboard_function_request: Pubkey,
    pub switchboard_function_request_escrow: Pubkey,
    pub competition_authority: Pubkey,

    // entries
    pub number_of_competitors: u128,
    pub number_of_competitors_settled: u128,
    // starts at zero and you have to settle everyone to know the winner
    pub total_score_settled: u128,
    // sum of all scores (when num users settled == num users)
    pub max_entries_per_competitor: u128, // set a max entry per competitior

    // giveaway details
    pub prize_amount: u128,
    pub prize_amount_settled: u128,
    pub prize_base: u128,

    pub winner_randomness: u128,
    pub prize_randomness: u128,
    // the max number in the prize_randomness request
    pub prize_randomness_max: u128,
    pub outstanding_unclaimed_winnings: u128,

    // scheduling variables
    pub round_number: u64,
    pub next_round_expiry_ts: i64,
    pub competition_expiry_ts: i64,
    // when competition ends, perpetual when == 0
    pub round_duration: u64,

    // selecting multiple winners
    pub number_of_winners: u32,
    pub number_of_winners_settled: u32,

    pub status: CompetitionRoundStatus,
    pub competition_authority_bump: u8,

    pub padding: [u8; 30],
}

impl Size for Competition {
    const SIZE: usize = 456 + 8;
}

const_assert_eq!(Competition::SIZE, std::mem::size_of::<Competition>() + 8);

impl Competition {
    pub fn update_status(&mut self, new_status: CompetitionRoundStatus) -> CompetitionResult {
        if new_status != CompetitionRoundStatus::Expired {
            let status_delta = (new_status as i32 + 1) - ((self.status as i32 + 1) % 4);
            validate!(
                status_delta == 1,
                ErrorCode::InvalidStatusUpdateDetected,
                "new status = {:?}, current status = {:?}",
                new_status,
                self.status
            )?;

            msg!(
                "updating Competition status: {:?} -> {:?}",
                self.status,
                new_status
            );
            self.status = new_status;
        }

        Ok(())
    }

    // calculate the end unix timestamp for round_number N
    pub fn calculate_round_end_ts(&self) -> DriftResult<i64> {
        self.next_round_expiry_ts
            .safe_add(self.round_duration.safe_mul(self.round_number)?.cast()?)
    }

    // calculate current round_number N
    pub fn calculate_next_round_expiry_ts(&self, now: i64) -> CompetitionResult<i64> {
        let next_round_expiry_ts = if now >= self.next_round_expiry_ts {
            self.next_round_expiry_ts.safe_add(
                self.round_duration
                    .safe_mul(
                        now.safe_sub(self.next_round_expiry_ts)?
                            .unsigned_abs()
                            .safe_div(self.round_duration)?
                            .safe_add(1)?,
                    )?
                    .cast::<i64>()?,
            )?
        } else {
            self.next_round_expiry_ts
        };

        Ok(next_round_expiry_ts)
    }

    pub fn expire(&mut self, now: i64) -> CompetitionResult {
        if self.competition_expiry_ts != 0 && self.competition_expiry_ts <= now {
            self.status = CompetitionRoundStatus::Expired;
        } else {
            return Err(ErrorCode::CompetitionRoundOngoing);
        }

        Ok(())
    }

    pub fn validate_round_settlement_hasnt_started(&self, now: i64) -> CompetitionResult {
        self.validate_round_is_active(now)?;

        validate!(
            self.number_of_competitors_settled == 0,
            ErrorCode::CompetitionInvariantIssue,
            "self.number_of_competitors_settled {} != 0",
            self.number_of_competitors_settled,
        )?;

        Ok(())
    }

    pub fn validate_round_is_active(&self, now: i64) -> CompetitionResult {
        validate!(
            self.status == CompetitionRoundStatus::Active,
            ErrorCode::CompetitionStatusNotActive,
            "Competition status = {:?} (should be Active)",
            self.status
        )?;

        validate!(
            !self.is_expired(now)?,
            ErrorCode::CompetitionExpired,
            "Competition Expired at unix_timestamp = {} ({} seconds ago)",
            self.competition_expiry_ts,
            now - self.competition_expiry_ts
        )?;
        Ok(())
    }

    pub fn is_expired(&self, now: i64) -> CompetitionResult<bool> {
        Ok(
            (self.competition_expiry_ts != 0 && self.competition_expiry_ts <= now)
                || self.status == CompetitionRoundStatus::Expired,
        )
    }

    pub fn validate_round_ready_for_settlement(&self, now: i64) -> CompetitionResult {
        self.validate_round_is_active(now)?;

        validate!(
            now >= self.next_round_expiry_ts,
            ErrorCode::CompetitionRoundOngoing,
            "round ends at unix_timestamp={} (seconds remaining {})",
            self.next_round_expiry_ts,
            self.next_round_expiry_ts - now
        )?;

        validate!(
            self.number_of_competitors_settled <= self.number_of_competitors,
            ErrorCode::CompetitionInvariantIssue,
            "self.number_of_competitors_settled={} > self.number_of_competitors={}",
            self.number_of_competitors_settled,
            self.number_of_competitors
        )?;

        Ok(())
    }

    pub fn validate_round_settlement_complete(&self) -> CompetitionResult {
        validate!(
            self.number_of_competitors == self.number_of_competitors_settled,
            ErrorCode::InvalidRoundSettlementDetected,
            "{} competitiors not not settled ({} != {})",
            self.number_of_competitors - self.number_of_competitors_settled,
            self.number_of_competitors,
            self.number_of_competitors_settled
        )?;

        validate!(
            self.status == CompetitionRoundStatus::WinnerSettlementComplete,
            ErrorCode::InvalidRoundSettlementDetected,
            "Competition status = {:?} (should be WinnerSettlementComplete)",
            self.status
        )?;

        Ok(())
    }

    pub fn validate_round_resolved(&self) -> CompetitionResult {
        validate!(
            self.number_of_competitors > 0
                && self.number_of_competitors == self.number_of_competitors_settled,
            ErrorCode::CompetitionRoundInSettlementPhase,
            "Competition round_number={:?} is still ongoing",
            self.round_number
        )?;

        Ok(())
    }

    pub fn validate_competitor_is_winner(&self, competitor: &Competitor) -> CompetitionResult {
        validate!(
            self.status == CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
                && self.winner_randomness != 0,
            ErrorCode::CompetitionWinnerNotDetermined,
            "CompetitionWinnerNotDetermined, winner randomness = {}",
            self.winner_randomness
        )?;

        // competitor account was settled and set to next round
        validate!(
            self.round_number < competitor.competition_round_number,
            ErrorCode::CompetitorHasWrongRoundNumber
        )?;

        // winning compeitor range is specified from (min_draw, max_draw]
        // this means winner_randomness must be > 0
        validate!(
            self.winner_randomness > competitor.min_draw
                && self.winner_randomness <= competitor.max_draw,
            ErrorCode::CompetitorNotWinner
        )?;

        Ok(())
    }

    pub fn competitor_can_be_settled(&self, competitor: &Competitor) -> CompetitionResult<bool> {
        let round_match = self.round_number == competitor.competition_round_number;

        Ok(round_match && competitor.is_active()?)
    }

    pub fn settle_competitor(
        &mut self,
        competitor: &mut Competitor,
        user_stats: &UserStats,
        now: i64,
    ) -> CompetitionResult {
        let previous_snapshot_score_before = competitor.previous_snapshot_score;
        self.validate_round_ready_for_settlement(now)?;

        if !self.competitor_can_be_settled(competitor)? {
            return Ok(()); // gracefully skip/fail
        }

        // skip unclaimed winners to give active competitors a higher probablity of winning
        if competitor.unclaimed_winnings == 0 {
            let round_score = competitor.calculate_round_score(user_stats)?;

            let round_score_capped = if self.max_entries_per_competitor > 0 {
                round_score.min(self.max_entries_per_competitor.cast()?)
            } else {
                round_score
            };

            // carry over half of capped round score as bonus
            competitor.bonus_score = round_score_capped.safe_div(2)?;

            let new_total_score_settled = self
                .total_score_settled
                .safe_add(round_score_capped.cast()?)?;

            competitor.min_draw = self.total_score_settled;
            competitor.max_draw = new_total_score_settled;

            self.total_score_settled = new_total_score_settled;
        } else {
            competitor.min_draw = self.total_score_settled;
            competitor.max_draw = self.total_score_settled;
        }

        validate!(
            competitor.competition_round_number == self.round_number,
            ErrorCode::CompetitionRoundNumberIssue,
            "competitor.competition_round_number = {:?} doesn't match competition = {}",
            competitor.competition_round_number,
            self.round_number
        )?;

        competitor.previous_snapshot_score = competitor.calculate_snapshot_score(&user_stats)?;
        competitor.competition_round_number = competitor.competition_round_number.safe_add(1)?;
        self.number_of_competitors_settled = self.number_of_competitors_settled.saturating_add(1);

        validate!(
            previous_snapshot_score_before <= competitor.previous_snapshot_score,
            ErrorCode::CompetitorSnapshotIssue
        )?;

        Ok(())
    }

    pub fn calculate_sponsor_max_prize(
        &self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult<u64> {

        let protocol_owned_shares_remaining = spot_market
            .insurance_fund
            .total_shares
            .safe_sub(spot_market.insurance_fund.user_shares)?
            .saturating_sub(self.outstanding_unclaimed_winnings);

        let protocol_owned_amount_remaining = if_shares_to_vault_amount(
            protocol_owned_shares_remaining,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )?;

        let max_prize = protocol_owned_amount_remaining
            .saturating_sub(self.sponsor_info.min_sponsor_amount)
            .safe_mul(self.sponsor_info.max_sponsor_fraction)?
            .safe_div(PERCENTAGE_PRECISION_U64)?;

        Ok(max_prize)
    }

    pub fn calculate_prize_buckets_and_ratios(
        self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult<([u128; 3], Vec<u128>)> {
        let max_prize: u128 = self
            .calculate_sponsor_max_prize(spot_market, vault_balance)?
            .cast()?;

        // prize ratios match [$1k, $5k, >= $10k] ratios, but lower prizes never exceed 1k, 5k
        let prize_buckets = [
            (1000 * QUOTE_PRECISION).min(max_prize / 10),
            (5000 * QUOTE_PRECISION).min(max_prize / 2),
            max_prize,
        ];

        let total_prize_bucket: u128 = prize_buckets.iter().sum();
        let mut ratios: Vec<u128> = vec![0; prize_buckets.len()]; // Using .len() to set the size

        for (i, &val) in prize_buckets.iter().enumerate() {
            if val > 0 {
                // round up for smaller prize buckets and down for largest
                if i < 2 {
                    ratios[i] = total_prize_bucket.safe_div_ceil(val)?;
                } else {
                    ratios[i] = total_prize_bucket.safe_div(val)?;
                }
            }
        }

        Ok((prize_buckets, ratios))
    }

    pub fn request_winner_and_prize_randomness(
        &mut self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult {
        self.validate_round_resolved()?;
        let (_, ratios) = self.calculate_prize_buckets_and_ratios(spot_market, vault_balance)?;

        let ratio_sum = ratios.iter().sum();
        self.prize_randomness_max = ratio_sum;

        self.update_status(CompetitionRoundStatus::WinnerAndPrizeRandomnessRequested)?;

        Ok(())
    }

    pub fn resolve_winner_and_prize_randomness(
        &mut self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult {
        self.validate_round_resolved()?;
        self.resolve_prize_amount(spot_market, vault_balance)?;
        self.update_status(CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete)?;

        Ok(())
    }

    pub fn calculate_prize_amount(
        &mut self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult<u128> {
        let (prize_buckets, ratios) =
            self.calculate_prize_buckets_and_ratios(spot_market, vault_balance)?;

        let ratio_sum: u128 = ratios.iter().sum();
        msg!("prize buckets: {:?}", prize_buckets);
        msg!("ratios: {:?}", ratios);
        msg!(
            "ratio_sum={} vs prize_randomness_max={}",
            ratio_sum,
            self.prize_randomness_max
        );

        // prize amounts changed since random draw request
        let draw = if ratio_sum < self.prize_randomness_max {
            let ranged_draw = self.prize_randomness % ratio_sum;
            msg!("prize_randomness range updated: {}", ranged_draw);
            ranged_draw
        } else {
            self.prize_randomness
        };

        let mut cumulative_ratio = 0;
        for (i, &prize_amount_i) in prize_buckets.iter().enumerate() {
            cumulative_ratio = cumulative_ratio.safe_add(ratios[i])?;
            if draw <= cumulative_ratio {
                let prize_amount = vault_amount_to_if_shares(
                    prize_amount_i.cast()?,
                    spot_market.insurance_fund.total_shares,
                    vault_balance,
                )?;

                return Ok(prize_amount);
            }
        }

        Err(ErrorCode::CompetitionWinnerNotDetermined)
    }

    pub fn resolve_prize_amount(
        &mut self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult {
        self.prize_amount = self.calculate_prize_amount(spot_market, vault_balance)?;
        self.prize_base = spot_market.insurance_fund.shares_base;

        Ok(())
    }

    pub fn calculate_next_winner_randomness(&mut self) -> CompetitionResult<u128> {
        let winner_randomness_offset: u128 = self
            .total_score_settled
            .safe_div(self.number_of_winners.cast()?)?
            .safe_add(1)?
            .safe_mul(
                self.prize_randomness
                    .safe_mul(self.number_of_winners_settled.cast()?)?,
            )?.saturating_add(17)
            ^ (self.prize_randomness)
            ^ (self.winner_randomness)
            << 3; // Bitwise XOR and left shift for added unpredictability

        let next_winner_randomness =
            self.winner_randomness.saturating_add(winner_randomness_offset) % (self.total_score_settled.saturating_add(1));

        msg!("winner_randomness: {} -> {} (offset={})", self.winner_randomness, next_winner_randomness, winner_randomness_offset);

        Ok(next_winner_randomness.max(1))
    }

    pub fn calculate_next_winner_prize_amount(&mut self) -> CompetitionResult<u128> {
        let winner_prize_amount = if self.number_of_winners <= 3 {
            // equal split of prize_amount when number_of_winners is low
            self.prize_amount.safe_div(self.number_of_winners.cast()?)?
        } else {
            // 50%, 20%, 15% of prize_amount for 1st, 2nd, 3rd respectively (in PERCENTAGE_PRECISION)
            let top_winner_prize_ratios: [u128; 3] = [500_000, 200_000, 150_000];

            // consolation pool is even split of the remainder for any winner past the 3rd
            let remainder = PERCENTAGE_PRECISION.safe_sub(top_winner_prize_ratios.iter().sum())?;

            let winner_prize_ratio =
                if (self.number_of_winners_settled as usize) < top_winner_prize_ratios.len() {
                    top_winner_prize_ratios[self.number_of_winners_settled as usize]
                } else {
                    remainder.safe_div(
                        (self
                            .number_of_winners
                            .safe_sub(top_winner_prize_ratios.len() as u32)?)
                        .cast()?,
                    )?
                };

            validate!(
                winner_prize_ratio <= top_winner_prize_ratios[0],
                ErrorCode::CompetitionInvariantIssue,
                "winner_prize_ratio = {}",
                winner_prize_ratio
            )?;

            self.prize_amount
                .safe_mul(winner_prize_ratio)?
                .safe_div(PERCENTAGE_PRECISION)?
        };

        let prize_amount_remaining = self.prize_amount.safe_sub(self.prize_amount_settled)?;

        Ok(winner_prize_amount.min(prize_amount_remaining))
    }

    pub fn settle_winner(
        &mut self,
        competitor: &mut Competitor,
        spot_market: &SpotMarket,
        now: i64,
    ) -> CompetitionResult {
        if self.number_of_winners == self.number_of_winners_settled {
            self.update_status(CompetitionRoundStatus::WinnerSettlementComplete)?;
            return Ok(());
        }

        self.validate_competitor_is_winner(competitor)?;

        if competitor.unclaimed_winnings != 0 {
            apply_rebase_to_competitor_unclaimed_winnings(competitor, spot_market)?;
        }

        if spot_market.insurance_fund.shares_base != self.prize_base {
            apply_rebase_to_competition_prize(self, spot_market)?;
        }

        let winner_prize_amount = self.calculate_next_winner_prize_amount()?;

        emit!(CompetitionRoundWinnerRecord {
            round_number: self.round_number,
            competitor: competitor.authority,
            min_draw: competitor.min_draw,
            max_draw: competitor.max_draw,
            total_score_settled: self.total_score_settled,
            number_of_competitors_settled: self.number_of_competitors_settled,

            prize_amount: winner_prize_amount,
            prize_base: self.prize_base,

            winner_placement: self.number_of_winners_settled,
            number_of_winners: self.number_of_winners,

            winner_randomness: self.winner_randomness,
            prize_randomness: self.prize_randomness,
            prize_randomness_max: self.prize_randomness_max,

            ts: now,
        });

        competitor.unclaimed_winnings = competitor
            .unclaimed_winnings
            .saturating_add(winner_prize_amount.cast()?);
        competitor.unclaimed_winnings_base = self.prize_base;

        // user splitting consolation pool by more than two retain tickets
        if self.number_of_winners_settled < 3 || self.number_of_winners <= 5 {
            competitor.bonus_score = 0; // reset bonus score to 0
        }

        self.outstanding_unclaimed_winnings = self
            .outstanding_unclaimed_winnings
            .saturating_add(winner_prize_amount.cast()?);
        self.prize_amount_settled = self.prize_amount_settled.safe_add(winner_prize_amount)?;
        self.number_of_winners_settled = self.number_of_winners_settled.safe_add(1)?;

        validate!(
            self.prize_amount_settled <= self.prize_amount
                && self.number_of_winners_settled <= self.number_of_winners,
            ErrorCode::CompetitionInvariantIssue,
            "{} / {} winners with {} / {} prize settled",
            self.number_of_winners_settled,
            self.number_of_winners,
            self.prize_amount_settled,
            self.prize_amount
        )?;

        if self.number_of_winners == self.number_of_winners_settled {
            self.update_status(CompetitionRoundStatus::WinnerSettlementComplete)?;
        } else {
            self.winner_randomness = self.calculate_next_winner_randomness()?; // update randomness for next winner to settle
        }

        Ok(())
    }

    pub fn reset_round(&mut self, now: i64) -> CompetitionResult {
        self.validate_round_settlement_complete()?;

        // necessary
        self.number_of_winners_settled = 0;
        self.total_score_settled = 0;
        self.number_of_competitors_settled = 0;
        self.round_number = self.round_number.safe_add(1)?;
        self.next_round_expiry_ts = self.calculate_next_round_expiry_ts(now)?;

        // 'nice to clear'
        self.winner_randomness = 0;
        self.prize_randomness = 0;
        self.prize_randomness_max = 0;
        self.prize_amount = 0;
        self.prize_amount_settled = 0;

        self.update_status(CompetitionRoundStatus::Active)?;

        Ok(())
    }
}
