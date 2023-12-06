use drift::math::constants::PRICE_PRECISION;
use drift::state::spot_market::SpotMarket;

use crate::error::{CompetitionResult, ErrorCode};

use crate::state::{Competition, Competitor};
use anchor_lang::prelude::*;
use drift::math::casting::Cast;
use drift::math::safe_math::SafeMath;
use drift::validate;

#[cfg(test)]
use drift::error::DriftResult;

#[cfg(test)]
pub fn get_test_sample_draw(min: u128, max: u128) -> DriftResult<u128> {
    // do a fake random draw

    assert!(min <= max);
    let random_number = (max - min) / 2 + min; // todo: replace with VRF

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

        msg!(
            "rebasing competition base: {} -> {} ",
            competition.prize_base,
            spot_market.insurance_fund.shares_base,
        );

        let old_if_shares = competition.prize_amount;
        let new_if_shares = old_if_shares.safe_div(rebase_divisor)?;

        competition.prize_amount = new_if_shares;
        competition.prize_amount_settled =
            competition.prize_amount_settled.safe_div(rebase_divisor)?;
        competition.outstanding_unclaimed_winnings = competition
            .outstanding_unclaimed_winnings
            .safe_div(rebase_divisor)?;
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
            "rebasing unclaimed_winnings_base: {} -> {} ",
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

pub fn calculate_revenue_pool_deposit_tokens_from_entries(
    entries: u64,
    spot_market: &SpotMarket,
) -> CompetitionResult<u64> {
    let quote_divisor = 20000; // this is .00005 quote

    let strict_price = spot_market
        .historical_oracle_data
        .last_oracle_price_twap_5min
        .min(spot_market.historical_oracle_data.last_oracle_price)
        .max(1)
        .cast::<u128>()?;
    let deposit_tokens = entries
        .cast::<u128>()?
        .safe_mul(PRICE_PRECISION)?
        .safe_mul(10_u128.pow(spot_market.decimals))?
        .safe_div_ceil(strict_price)?
        .safe_div_ceil(quote_divisor)?;

    Ok(deposit_tokens.cast()?)
}
