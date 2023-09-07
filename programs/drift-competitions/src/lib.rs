use anchor_lang::prelude::*;
use instructions::*;

mod error;
mod instructions;
mod state;
mod tests;
mod utils;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod drift_competitions {
    use super::*;

    pub fn initialize_competition<'info>(
        ctx: Context<'_, '_, '_, 'info, InitializeCompetition<'info>>,
        params: CompetitionParams,
    ) -> Result<()> {
        instructions::initialize_competition(ctx, params)
    }
}
