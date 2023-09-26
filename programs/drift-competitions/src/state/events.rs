use anchor_lang::prelude::*;
use borsh::{BorshDeserialize, BorshSerialize};

#[event]
#[derive(Default)]
pub struct CompetitionRoundWinnerRecord {
    pub round_number: u64,
    pub competitor: Pubkey,

    pub min_draw: u128,
    pub max_draw: u128,

    pub number_of_competitors_settled: u128,

    pub winner_randomness: u128,
    pub total_score_settled: u128,

    pub prize_randomness: u128,
    pub prize_randomness_max: u128,

    pub prize_amount: u128,
    pub prize_base: u128,

    pub ts: i64,
}
