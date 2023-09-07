use anchor_lang::prelude::*;
use static_assertions::const_assert_eq;
use crate::state::Size;

#[account(zero_copy)]
#[derive(Default, Eq, PartialEq, Debug)]
#[repr(C)]
pub struct Competition {
    pub name: [u8; 32],
    pub sponsor: Pubkey,
}

impl Size for Competition {
    const SIZE: usize = 64 + 8;
}

const_assert_eq!(Competition::SIZE, std::mem::size_of::<Competition>() + 8);