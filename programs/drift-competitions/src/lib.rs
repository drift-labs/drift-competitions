use anchor_lang::prelude::*;
use instructions::*;
use switchboard_solana::prelude::*;

mod error;
mod instructions;
mod signer_seeds;
mod state;
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

    pub fn update_switchboard_function<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateSwitchboardFunction<'info>>,
    ) -> Result<()> {
        instructions::update_switchboard_function(ctx)
    }

    // competitor ix
    pub fn initialize_competitor<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeCompetitor<'info>>,
    ) -> Result<()> {
        instructions::initialize_competitor(ctx)
    }

    pub fn claim_entry<'info>(ctx: Context<'_, '_, '_, 'info, ClaimEntry<'info>>) -> Result<()> {
        instructions::claim_entry(ctx)
    }

    pub fn claim_winnings<'info>(
        ctx: Context<'_, '_, '_, 'info, ClaimWinnings<'info>>,
        n_shares: Option<u64>,
    ) -> Result<()> {
        instructions::claim_winnings(ctx, n_shares)
    }

    // keeper ix

    pub fn settle_competitor<'info>(
        ctx: Context<'_, '_, '_, 'info, SettleCompetitor<'info>>,
    ) -> Result<()> {
        instructions::settle_competitor(ctx)
    }

    pub fn request_randomness<'info>(
        ctx: Context<'_, '_, '_, 'info, RequestRandomness<'info>>,
    ) -> Result<()> {
        instructions::request_randomness(ctx)
    }

    pub fn receive_randomness<'info>(
        ctx: Context<'_, '_, '_, 'info, ReceiveRandomness<'info>>,
        winner_randomness: u128,
        prize_randomness: u128,
    ) -> Result<()> {
        instructions::receive_randomness(ctx, winner_randomness, prize_randomness)
    }

    pub fn settle_winner<'info>(
        ctx: Context<'_, '_, '_, 'info, SettleWinner<'info>>,
    ) -> Result<()> {
        instructions::settle_winner(ctx)
    }
}
