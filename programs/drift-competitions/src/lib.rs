use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;
use instructions::*;

mod instructions;
mod state;

declare_id!("9FHbMuNCRRCXsvKEkA3V8xJmysAqkZrGfrppvUhGTq7x");

#[program]
pub mod drift_competitions {
    use super::*;

    pub fn initialize_competition<'info>(ctx: Context<'_, '_, '_, 'info, InitializeCompetition<'info>>, params: CompetitionParams) -> Result<()> {
        instructions::initialize_competition(ctx, params)
    }

    pub fn request_randomness<'info>(ctx: Context<'_, '_, '_, 'info, RequestRandomness<'info>>) -> Result<()> {
        instructions::request_randomness(ctx)
    }

    pub fn settle<'info>(ctx: Context<'_, '_, '_, 'info, Settle<'info>>, result: u32) -> Result<()> {
        instructions::settle(ctx, result)
    }

    pub fn close_randomness_request<'info>(ctx: Context<'_, '_, '_, 'info, CloseRandomness<'info>>) -> Result<()> {
        instructions::close_randomness_request(ctx)
    }
}
