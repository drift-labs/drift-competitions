use anchor_lang::prelude::*;
use switchboard_solana::prelude::*;
use instructions::*;

mod error;
mod instructions;
mod state;
mod signer_seeds;
mod utils;

#[cfg(test)]
mod tests;

declare_id!("9FHbMuNCRRCXsvKEkA3V8xJmysAqkZrGfrppvUhGTq7x");

#[program]
pub mod drift_competitions {
    use super::*;

    // sponsor ix
    pub fn initialize_competition<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeCompetition<'info>>,
        params: CompetitionParams,
    ) -> Result<()> {
        instructions::initialize_competition(ctx, params)
    }

    pub fn update_competition<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateCompetition<'info>>,
        params: UpdateCompetitionParams,
    ) -> Result<()> {
        instructions::update_competition(ctx, params)
    }

    pub fn update_switchboard_function<'info>(ctx: Context<'_, '_, '_, 'info, UpdateSwitchboardFunction<'info>>) -> Result<()> {
        instructions::update_switchboard_function(ctx)
    }

    // competitor ix
    pub fn initialize_competitor<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeCompetitor<'info>>,
    ) -> Result<()> {
        instructions::initialize_competitor(ctx)
    }

    // keeper ix

    pub fn settle_competitor<'info>(
        ctx: Context<'_, '_, '_, 'info, SettleCompetitor<'info>>,
    ) -> Result<()> {
        instructions::settle_competitor(ctx)
    }

    pub fn settle_competition<'info>(
        ctx: Context<'_, '_, '_, 'info, SettleCompetition<'info>>,
    ) -> Result<()> {
        instructions::settle_competition(ctx)
    }

    pub fn request_randomness<'info>(ctx: Context<'_, '_, '_, 'info, RequestRandomness<'info>>) -> Result<()> {
        instructions::request_randomness(ctx)
    }

    pub fn receive_randomness<'info>(ctx: Context<'_, '_, '_, 'info, ReceiveRandomness<'info>>, winner_randomness: u128, prize_randomness: u128) -> Result<()> {
        instructions::receive_randomness(ctx, winner_randomness, prize_randomness)
    }
}
