use drift::{error::DriftResult, state::spot_market::SpotMarket};

use crate::error::{CompetitionResult, ErrorCode};

use crate::state::{Competition, Competitor};
use anchor_lang::prelude::*;
use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;
// use solana_program::msg;
use drift::validate;

pub fn get_random_draw(max: u128) -> DriftResult<u128> {
    let random_number = max / 2; // todo: replace with VRF

    Ok(random_number)
}

pub fn apply_rebase_to_competition_prize(
    competition: &mut Competition,
    spot_market: &SpotMarket,
) -> CompetitionResult {
    if spot_market.insurance_fund.shares_base != competition.prize_base {
        validate!(
            spot_market.insurance_fund.shares_base > competition.prize_base,
            ErrorCode::InvalidIFRebase,
            "Rebase expo out of bounds"
        )?;

        let expo_diff =
            (spot_market.insurance_fund.shares_base - competition.prize_base).cast::<u32>()?;

        let rebase_divisor = 10_u128.pow(expo_diff);

        competition.prize_base = spot_market.insurance_fund.shares_base;

        let old_if_shares = competition.prize_base;
        let new_if_shares = old_if_shares.safe_div(rebase_divisor)?;

        competition.prize_amount = new_if_shares;
    }
    Ok(())
}

pub fn apply_rebase_to_competitor_unclaimed_winnings(
    competitor: &mut Competitor,
    spot_market: &SpotMarket,
) -> CompetitionResult {
    if spot_market.insurance_fund.shares_base != competitor.unclaimed_winnings_base {
        validate!(
            spot_market.insurance_fund.shares_base > competitor.unclaimed_winnings_base,
            ErrorCode::InvalidIFRebase,
            "Rebase expo out of bounds"
        )?;

        let expo_diff = (spot_market.insurance_fund.shares_base
            - competitor.unclaimed_winnings_base)
            .cast::<u32>()?;

        let rebase_divisor = 10_u64.pow(expo_diff);

        msg!(
            "rebasing insurance fund stake: base: {} -> {} ",
            competitor.unclaimed_winnings_base,
            spot_market.insurance_fund.shares_base,
        );

        competitor.unclaimed_winnings_base = spot_market.insurance_fund.shares_base;

        let old_if_shares = competitor.unclaimed_winnings;
        let new_if_shares = old_if_shares.safe_div(rebase_divisor)?;

        competitor.unclaimed_winnings = new_if_shares;
    }

    Ok(())
}
