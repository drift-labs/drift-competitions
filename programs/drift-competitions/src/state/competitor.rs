use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use crate::state::Size;

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
}

impl Size for Competitor {
    const SIZE: usize = 104 + 8;
}

const_assert_eq!(Competitor::SIZE, std::mem::size_of::<Competitor>() + 8);