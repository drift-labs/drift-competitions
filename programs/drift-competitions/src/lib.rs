use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;
use instructions::*;

mod instructions;
mod state;
mod signer_seeds;

declare_id!("9FHbMuNCRRCXsvKEkA3V8xJmysAqkZrGfrppvUhGTq7x");

#[program]
pub mod drift_competitions {
    use super::*;

    pub fn initialize_competition<'info>(ctx: Context<'_, '_, '_, 'info, InitializeCompetition<'info>>, params: CompetitionParams) -> Result<()> {
        instructions::initialize_competition(ctx, params)
    }

    pub fn update_switchboard_function<'info>(ctx: Context<'_, '_, '_, 'info, UpdateSwitchboardFunction<'info>>) -> Result<()> {
        instructions::update_switchboard_function(ctx)
    }

    pub fn request_randomness<'info>(ctx: Context<'_, '_, '_, 'info, RequestRandomness<'info>>) -> Result<()> {
        instructions::request_randomness(ctx)
    }

    pub fn receive_randomness<'info>(ctx: Context<'_, '_, '_, 'info, Settle<'info>>, winner_randomness: u128, prize_randomness: u128) -> Result<()> {
        instructions::receive_randomness(ctx, winner_randomness, prize_randomness)
    }
}
