use anchor_lang::prelude::*;
use borsh::BorshSerialize;

use super::CompetitorStatus;

#[event]
#[derive(Default)]
pub struct CompetitionRoundSummaryRecord {
    pub competition: Pubkey,
    pub round_number: u64,
    pub round_start_ts: i64,
    pub round_end_ts: i64,

    pub prize_placement: u32,
    pub prize_odds_numerator: u128,
    pub prize_randomness: u128,
    pub prize_randomness_max: u128,
    pub max_prize_bucket_value: u64,

    pub prize_amount: u128,
    pub prize_value: u64,
    pub prize_base: u128,

    pub number_of_winners: u32,
    pub number_of_competitors_settled: u128,
    pub total_score_settled: u128,

    pub insurance_vault_balance: u64,
    pub protocol_if_shares: u128,
    pub total_if_shares: u128,

    pub ts: i64,
}

#[event]
#[derive(Default)]
pub struct CompetitionRoundWinnerRecord {
    pub round_number: u64,
    pub competitor: Pubkey,
    pub competition: Pubkey,
    pub competitor_authority: Pubkey,

    pub min_draw: u128,
    pub max_draw: u128,

    pub winner_placement: u32,
    pub number_of_winners: u32,
    pub number_of_competitors_settled: u128,

    pub winner_randomness: u128,
    pub total_score_settled: u128,

    pub prize_randomness: u128,
    pub prize_randomness_max: u128,

    pub prize_amount: u128,
    pub prize_base: u128,
    pub prize_value: u64,

    pub ts: i64,
}

#[event]
#[derive(Default)]
pub struct CompetitorSettledRecord {
    pub round_number: u64,
    pub competitor: Pubkey,
    pub competition: Pubkey,
    pub competitor_authority: Pubkey,

    pub status: CompetitorStatus,
    pub unclaimed_winnings: u64,

    pub min_draw: u128,
    pub max_draw: u128,
    pub bonus_score_before: u64,
    pub bonus_score_after: u64,
    pub previous_snapshot_score_before: u64,
    pub snapshot_score: u64,

    pub ts: i64,
}
