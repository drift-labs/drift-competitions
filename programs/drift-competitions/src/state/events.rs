use anchor_lang::prelude::*;
use borsh::BorshSerialize;

use super::CompetitorStatus;

#[event]
#[derive(Default)]
pub struct CompetitionRoundSummaryRecord {
    pub competition: Pubkey,
    pub round_number: u64, // the round number ending 
    pub round_start_ts: i64, // the round expiry ts - round duration
    pub round_end_ts: i64, // the round expiry ts

    pub prize_placement: u32, // which prize bucket was selected (zero indexed). highest is 2 which is max_prize
    pub prize_odds_numerator: u128, // numerator of odds of the selected bucket (e.g. odds = prize_odds_numerator / prize_randomness_max)
    pub prize_randomness: u128, // random draw from [0, prizeRandomnessMax] to decide prize bucket
    pub prize_randomness_max: u128, // max number for prize draw 
    pub max_prize_bucket_value: u64, // snapshot of max prize in prize buckets

    pub prize_amount: u128, // prize amount for unclaimed winnings (unit: if_shares)
    pub prize_value: u64, // token value of the if_shares at the time of settling winner
    pub prize_base: u128, // prize if_shares base unclaimed winnings

    pub number_of_winners: u32, // number of winners for competition
    pub number_of_competitors_settled: u128, // count of competitors who were settled to competition
    pub total_score_settled: u128, // total number of entries settled to competition

    pub insurance_vault_balance: u64, // snapshot of if token balance
    pub protocol_if_shares: u128, // snapshot of protocol-owned shares in spot market insurance fund
    pub total_if_shares: u128, // snapshot of total shares in spot market insurance fund 

    pub ts: i64, // unix timestamp this record was emitted
}

#[event]
#[derive(Default)]
pub struct CompetitionRoundWinnerRecord {
    pub round_number: u64, // count of rounds for this competition
    pub competitor: Pubkey, // public key of corresponding competitor account
    pub competition: Pubkey, // public key of corresponding competition account
    pub competitor_authority: Pubkey, // public key of authority of competitior

    pub min_draw: u128, // competitior lowest numbered entry (exclusive)
    pub max_draw: u128, // competitior highest numbered entry

    pub winner_placement: u32, // order of the win. from [0, number_of_winners)
    pub number_of_winners: u32, // number of winners as specified by the competition
    pub number_of_competitors_settled: u128, // count of competitors who were settled to competition

    pub winner_randomness: u128, // random (or derived) value for selecting winner. from (min_draw, max_draw].
    pub total_score_settled: u128, // total entries across all settled competitors

    pub prize_randomness: u128, // random draw from [0, prizeRandomnessMax] to decide prize bucket
    pub prize_randomness_max: u128, // max number for prize draw 

    pub prize_amount: u128, // prize amount for unclaimed winnings (unit: if_shares)
    pub prize_base: u128, // prize if_shares base unclaimed winnings
    pub prize_value: u64, // token value of the if_shares at the time of settling winner

    pub ts: i64, // unix timestamp this record was emitted
}

#[event]
#[derive(Default)]
pub struct CompetitorSettledRecord {
    pub round_number: u64, // count of rounds for this competition
    pub competitor: Pubkey, // public key of corresponding competitor account
    pub competition: Pubkey, // public key of corresponding competition account
    pub competitor_authority: Pubkey, // public key of authority of competitior

    pub status: CompetitorStatus, // status of whether the competitior is in good standing
    pub unclaimed_winnings: u64, // competitors current unclaimed winnings (they won't be considered for this draw if non-zero)

    pub min_draw: u128, // competitior lowest numbered entry (exclusive)
    pub max_draw: u128, // competitior highest numbered entry
    pub bonus_score_before: u64, // bonus score before settlement
    pub bonus_score_after: u64, // bonus score after settlement
    pub previous_snapshot_score_before: u64, // previous round's score derived from user stats snapshot
    pub snapshot_score: u64, // current score derived from user stats snapshot

    pub ts: i64,
}
