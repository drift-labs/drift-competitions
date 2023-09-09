use crate::state::Size;
use crate::utils::get_random_draw;
use drift::{
    error::DriftResult,
    math::constants::{PERCENTAGE_PRECISION_U64, QUOTE_PRECISION},
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

use drift::math::insurance::{if_shares_to_vault_amount, vault_amount_to_if_shares};

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug)]
pub enum CompetitionType {
    Sweepstakes,
}

impl Default for CompetitionType {
    fn default() -> Self {
        CompetitionType::Sweepstakes
    }
}

#[derive(Clone, Copy, BorshSerialize, BorshDeserialize, PartialOrd, Ord, PartialEq, Eq, Debug)]
pub enum CompetitionRoundStatus {
    Active = 1,
    PrizeDrawComplete = 2,
    PrizeAmountComplete = 3,
    WinnerDrawComplete = 4,
    WinnerSettlementComplete = 5,
    Expired = 6,
}

impl Default for CompetitionRoundStatus {
    fn default() -> Self {
        CompetitionRoundStatus::Active
    }
}

#[account(zero_copy)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct SponsorInfo {
    pub sponsor: Pubkey,
    pub min_sponsor_amount: u64,   // always leave this amount for sponsor
    pub max_sponsor_fraction: u64, // only take this percent of gain above the min amount
}

#[account(zero_copy)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Competition {
    pub name: [u8; 32],
    pub competition_type: CompetitionType,
    pub status: CompetitionRoundStatus,

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

    pub prize_draw: u128,
    pub prize_amount: u128,
    pub prize_base: u128,
    pub winning_draw: u128,

    pub sponsor_info: SponsorInfo,
}

impl Size for Competition {
    const SIZE: usize = 248 + 8;
}

const_assert_eq!(Competition::SIZE, std::mem::size_of::<Competition>() + 8);

impl Competition {
    pub fn update_status(&mut self, new_status: CompetitionRoundStatus) -> CompetitionResult {
        if new_status != CompetitionRoundStatus::Expired {
            if new_status == CompetitionRoundStatus::Active {
                validate!(
                    self.status == CompetitionRoundStatus::WinnerSettlementComplete,
                    ErrorCode::InvalidStatusUpdateDetected
                )?;
                self.status = new_status;
            } else {
                validate!(
                    (new_status as i32) - (self.status as i32) == 1,
                    ErrorCode::InvalidStatusUpdateDetected
                )?;
                validate!(
                    new_status > self.status,
                    ErrorCode::InvalidStatusUpdateDetected
                )?;
                self.status = new_status;
            }
        }

        Ok(())
    }

    // calculate the end unix timestamp for round_number N
    pub fn calculate_round_end_ts(&self) -> DriftResult<i64> {
        self.first_round_expiry_ts
            .safe_add(self.round_duration.safe_mul(self.round_number)?.cast()?)
    }

    pub fn expire(&mut self, now: i64) -> CompetitionResult {
        if self.competition_expiry_ts != 0 && self.competition_expiry_ts <= now {
            self.status = CompetitionRoundStatus::Expired;
        }

        Ok(())
    }

    pub fn validate_round_ready_for_settlement(&self, now: i64) -> CompetitionResult {
        validate!(
            self.competition_expiry_ts == 0
                || self.competition_expiry_ts > now
                || self.status == CompetitionRoundStatus::Expired,
            ErrorCode::CompetitionExpired
        )?;

        validate!(
            now >= self.calculate_round_end_ts()?,
            ErrorCode::CompetitionRoundOngoing
        )?;

        validate!(
            self.status == CompetitionRoundStatus::Active,
            ErrorCode::CompetitionStatusNotActive
        )?;

        Ok(())
    }

    pub fn validate_round_settlement_complete(&self) -> CompetitionResult {
        validate!(
            self.number_of_competitors == self.number_of_competitors_settled,
            ErrorCode::InvalidRoundSettlementDetected
        )?;

        validate!(
            self.status == CompetitionRoundStatus::WinnerSettlementComplete,
            ErrorCode::InvalidRoundSettlementDetected
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

    pub fn settle_competitor(
        &mut self,
        competitor: &mut Competitor,
        user_stats: &UserStats,
        now: i64,
    ) -> CompetitionResult {
        self.validate_round_ready_for_settlement(now)?;

        if competitor.unclaimed_winnings == 0 {
            let round_score = competitor.calculate_round_score(user_stats)?;

            let new_total_score_settled = self.total_score_settled.safe_add(round_score.cast()?)?;

            competitor.min_draw = self.total_score_settled;
            competitor.max_draw = new_total_score_settled;

            self.total_score_settled = new_total_score_settled;
        }

        self.number_of_competitors_settled = self.number_of_competitors_settled.saturating_add(1);

        Ok(())
    }

    pub fn calculate_sponsor_max_prize(
        &self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult<u64> {
        let protocol_owned_shares = spot_market
            .insurance_fund
            .total_shares
            .safe_sub(spot_market.insurance_fund.user_shares)?;

        let protocol_owned_amount = if_shares_to_vault_amount(
            protocol_owned_shares,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )?;

        let max_prize = protocol_owned_amount
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

        let prize_buckets = [
            (1000 * QUOTE_PRECISION).min(max_prize / 10),
            (5000 * QUOTE_PRECISION).min(max_prize / 2),
            max_prize,
        ];

        let total_prize_bucket: u128 = prize_buckets.iter().sum();
        let mut ratios: Vec<u128> = vec![0; prize_buckets.len()]; // Using .len() to set the size

        for (i, &val) in prize_buckets.iter().enumerate() {
            if val > 0 {
                ratios[i] = total_prize_bucket.safe_div_ceil(val)?;
            }
        }

        Ok((prize_buckets, ratios))
    }

    pub fn resolve_prize_draw(
        &mut self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult {
        self.validate_round_resolved()?;
        let (_, ratios) = self.calculate_prize_buckets_and_ratios(spot_market, vault_balance)?;
        self.prize_draw = get_random_draw(ratios.iter().sum())?;

        self.update_status(CompetitionRoundStatus::PrizeDrawComplete)?;

        Ok(())
    }

    pub fn resolve_prize_amount(
        &mut self,
        spot_market: &SpotMarket,
        vault_balance: u64,
    ) -> CompetitionResult {
        let (prize_buckets, ratios) =
            self.calculate_prize_buckets_and_ratios(spot_market, vault_balance)?;

        let mut cumulative_ratio = 0;
        for (i, &prize_amount_i) in prize_buckets.iter().enumerate() {
            cumulative_ratio = cumulative_ratio.safe_add(ratios[i])?;
            if self.prize_draw <= cumulative_ratio {
                self.prize_amount = vault_amount_to_if_shares(
                    prize_amount_i.cast()?,
                    spot_market.insurance_fund.total_shares,
                    vault_balance,
                )?;

                self.prize_base = spot_market.insurance_fund.shares_base;
            }
        }

        self.update_status(CompetitionRoundStatus::PrizeAmountComplete)?;

        Ok(())
    }

    pub fn resolve_winning_draw(&mut self) -> CompetitionResult {
        self.validate_round_resolved()?;

        self.winning_draw = get_random_draw(self.total_score_settled)?;
        self.update_status( CompetitionRoundStatus::WinnerDrawComplete)?;

        Ok(())
    }

    pub fn settle_winner(&mut self, competitor: &mut Competitor) -> CompetitionResult {
        validate!(
            self.status == CompetitionRoundStatus::WinnerDrawComplete,
            ErrorCode::CompetitionWinnerNotDetermined
        )?;

        validate!(
            self.round_number == competitor.competition_round_number,
            ErrorCode::CompetitorHasWrongRoundNumber
        )?;

        validate!(
            self.winning_draw >= competitor.min_draw && self.winning_draw < competitor.max_draw,
            ErrorCode::CompetitorNotWinner
        )?;

        validate!(
            competitor.unclaimed_winnings == 0,
            ErrorCode::CompetitorNotQualified
        )?;

        competitor.unclaimed_winnings = self.prize_draw.cast()?;
        competitor.unclaimed_winnings_base = self.prize_base;

        self.update_status( CompetitionRoundStatus::WinnerSettlementComplete)?;


        Ok(())
    }

    pub fn reset_round(&mut self) -> CompetitionResult {
        self.validate_round_settlement_complete()?;

        self.total_score_settled = 0;
        self.number_of_competitors_settled = 0;
        self.round_number = self.round_number.safe_add(1)?;

        self.update_status(  CompetitionRoundStatus::Active)?;

        Ok(())
    }
}
