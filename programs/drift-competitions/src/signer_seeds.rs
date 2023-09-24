use crate::Pubkey;

pub fn get_function_authority_seeds<'a>(competition: &'a Pubkey, bump: &'a u8) -> [&'a [u8]; 3] {
    [
        b"competition_authority".as_ref(),
        competition.as_ref(),
        bytemuck::bytes_of(bump),
    ]
}
