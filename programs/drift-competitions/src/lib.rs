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
}
