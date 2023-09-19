use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use crate::state::Size;
use drift_macros::assert_no_slop;

#[assert_no_slop]
#[account(zero_copy)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Competition {
    pub name: [u8; 32],
    pub sponsor: Pubkey,

    pub switchboard_function: Pubkey,
    pub switchboard_function_request: Pubkey,
    pub switchboard_function_request_escrow: Pubkey,

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

    pub padding: [u128; 16],
}

impl Size for Competition {
    const SIZE: usize = 512 + 8;
}

const_assert_eq!(Competition::SIZE, std::mem::size_of::<Competition>() + 8);