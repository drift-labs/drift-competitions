use anchor_lang::prelude::*;
use instructions::*;
use state::CompetitorStatus;

mod error;
mod instructions;
mod signer_seeds;
pub mod state;
mod utils;

#[cfg(test)]
mod tests;

declare_id!("DraWMeQX9LfzQQSYoeBwHAgM5JcqFkgrX7GbTfjzVMVL");

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

    pub fn update_competitor_status<'info>(
        ctx: Context<'_, '_, '_, 'info, UpdateCompetitorStatus<'info>>,
        new_status: CompetitorStatus,
    ) -> Result<()> {
        instructions::update_competitor_status(ctx, new_status)
    }

    // competitor ix
    pub fn initialize_competitor<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeCompetitor<'info>>,
    ) -> Result<()> {
        instructions::initialize_competitor(ctx)
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

    pub fn settle_winner<'info>(
        ctx: Context<'_, '_, '_, 'info, SettleWinner<'info>>,
    ) -> Result<()> {
        instructions::settle_winner(ctx)
    }
}
