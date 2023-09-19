#[cfg(test)]
mod competition_fcn {
    use crate::state::{Competition, CompetitionRoundStatus, Competitor};
    use drift::{
        math::{
            constants::{PERCENTAGE_PRECISION, PERCENTAGE_PRECISION_U64, QUOTE_PRECISION},
            insurance::if_shares_to_vault_amount,
        },
        state::{
            insurance_fund_stake::InsuranceFundStake, spot_market::SpotMarket, user::UserStats,
        },
    };

    #[test]
    fn test_competition_settlement() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.first_round_expiry_ts = now + 60;
        sweepstakes.round_duration = 60;
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);

        sweepstakes.number_of_competitors = 2;
        let comp1 = &mut Competitor::default();
        comp1.claim_entry().unwrap();

        let comp2 = &mut Competitor::default();
        comp2.claim_entry().unwrap();
        comp2.bonus_score += 2;

        let us: &UserStats = &UserStats::default();

        assert!(sweepstakes.settle_competitor(comp1, us, now).is_err());
        assert!(sweepstakes.settle_competitor(comp2, us, now).is_err());

        now += 60;

        assert!(sweepstakes.reset_round().is_err());
        assert!(sweepstakes.resolve_winning_draw().is_err());

        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 1);
        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 1); // resettle a competitor doesnt increment

        sweepstakes.settle_competitor(comp2, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 2);

        assert_eq!(comp1.min_draw, 0);
        assert_eq!(comp1.max_draw, 1);

        assert_eq!(comp2.min_draw, 1);
        assert_eq!(comp2.max_draw, 4);

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 1100;
        spot_market.insurance_fund.user_shares = 1000;

        let vault_balance: u64 = (1580000 * QUOTE_PRECISION) as u64;

        sweepstakes
            .resolve_prize_draw(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::PrizeDrawComplete
        );
        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::PrizeAmountComplete
        );

        sweepstakes.resolve_winning_draw().unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerDrawComplete
        );

        assert!(sweepstakes.reset_round().is_err());

        assert_eq!(sweepstakes.winning_draw, 2);

        assert!(sweepstakes.settle_winner(comp1, &spot_market).is_err());
        sweepstakes.settle_winner(comp2, &spot_market).unwrap();
        assert!(sweepstakes.settle_winner(comp2, &spot_market).is_err()); // cannot settle twice
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );

        assert_eq!(comp2.unclaimed_winnings, sweepstakes.prize_amount as u64);

        sweepstakes.reset_round().unwrap();
        assert_eq!(sweepstakes.round_number, 1);
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);
    }

    #[test]
    fn test_competition_expiry() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.first_round_expiry_ts = now + 60;
        sweepstakes.round_duration = 60;
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);

        sweepstakes.competition_expiry_ts = now + 88;

        sweepstakes.number_of_competitors = 2;
        let comp1 = &mut Competitor::default();
        comp1.claim_entry().unwrap();

        let comp2 = &mut Competitor::default();

        assert!(sweepstakes.expire(now).is_err());
        now += 5;
        assert!(sweepstakes.expire(now).is_err());
        now += 85;

        assert!(sweepstakes.reset_round().is_err());
        assert!(sweepstakes.resolve_winning_draw().is_err());

        let us: &UserStats = &UserStats::default();

        assert!(sweepstakes.settle_competitor(comp1, us, now).is_err());
        assert!(sweepstakes.settle_competitor(comp2, us, now).is_err());

        sweepstakes.expire(now).unwrap();

        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Expired);
    }

    #[test]
    fn test_compeitition_prize_rebases() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.first_round_expiry_ts = now + 60;
        sweepstakes.round_duration = 60;
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);
        sweepstakes.sponsor_info.max_sponsor_fraction = PERCENTAGE_PRECISION_U64 / 2;
        sweepstakes.number_of_competitors = 2;
        let comp1 = &mut Competitor::default();
        comp1.claim_entry().unwrap();

        let comp2 = &mut Competitor::default();
        comp2.claim_entry().unwrap();
        comp2.bonus_score += 2;

        let us: &UserStats = &UserStats::default();

        assert!(sweepstakes.settle_competitor(comp1, us, now).is_err());
        assert!(sweepstakes.settle_competitor(comp2, us, now).is_err());

        now += 60;

        assert!(sweepstakes.reset_round().is_err());
        assert!(sweepstakes.resolve_winning_draw().is_err());

        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 1);
        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 1); // resettle a competitor doesnt increment

        sweepstakes.settle_competitor(comp2, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 2);

        assert_eq!(comp1.min_draw, 0);
        assert_eq!(comp1.max_draw, 1);

        assert_eq!(comp2.min_draw, 1);
        assert_eq!(comp2.max_draw, 4);

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 1100 * PERCENTAGE_PRECISION;
        spot_market.insurance_fund.user_shares = 1000 * PERCENTAGE_PRECISION;
        spot_market.insurance_fund.shares_base = 1;
        let vault_balance: u64 = (1580000 * QUOTE_PRECISION) as u64;

        sweepstakes
            .resolve_prize_draw(&spot_market, vault_balance)
            .unwrap();
        // spot_market.insurance_fund.shares_base = 2;

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::PrizeDrawComplete
        );

        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 71818181818]);
        assert_eq!(ratios, [78, 16, 2]);

        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::PrizeAmountComplete
        );
        assert!(sweepstakes.prize_amount > 0);
        assert_eq!(sweepstakes.prize_amount, 696202); // if shares
        assert_eq!(spot_market.insurance_fund.total_shares, 1100000000);

        let prize_amount_quote = if_shares_to_vault_amount(
            sweepstakes.prize_amount,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )
        .unwrap();

        assert!(prize_amount_quote as u128 <= prize_buckets[0]);
        assert_eq!(prize_amount_quote, 999999236); // slightly less from IF share rounding

        spot_market.insurance_fund.shares_base = 3;

        sweepstakes.resolve_winning_draw().unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerDrawComplete
        );
        spot_market.insurance_fund.shares_base = 4;
        assert_eq!(sweepstakes.prize_amount, 696202);

        assert!(sweepstakes.reset_round().is_err());
        spot_market.insurance_fund.shares_base = 5;
        assert_eq!(sweepstakes.winning_draw, 2);

        assert!(sweepstakes.settle_winner(comp1, &spot_market).is_err());

        assert_eq!(sweepstakes.prize_amount, 696202);
        assert_eq!(sweepstakes.prize_base, 1);

        sweepstakes.settle_winner(comp2, &spot_market).unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );

        assert_eq!(comp2.unclaimed_winnings_base, 5);
        assert_eq!(comp2.unclaimed_winnings, 69);
        assert_eq!(sweepstakes.prize_amount, 69); //rebased by 4 zeros

        assert!(sweepstakes.settle_winner(comp2, &spot_market).is_err()); // cannot settle twice
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );
        spot_market.insurance_fund.shares_base = 6;

        assert_eq!(comp2.competition_round_number, 1); // already on next round
        assert_eq!(comp2.unclaimed_winnings_base, 5);

        assert_eq!(comp2.unclaimed_winnings, 69);

        sweepstakes.reset_round().unwrap();
        assert_eq!(sweepstakes.round_number, 1);
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);
        assert!(comp2.unclaimed_winnings > 0);

        let mut insurance_fund_stake = InsuranceFundStake::default();
        assert!(comp2
            .claim_winnings(&spot_market, &mut insurance_fund_stake)
            .is_err()); // insurance_fund_stake base mismatch w/ spot_market
        assert!(comp2.unclaimed_winnings > 0);

        insurance_fund_stake.if_base = 5; // match unclaimed_winnings_base
        assert!(comp2
            .claim_winnings(&spot_market, &mut insurance_fund_stake)
            .is_err()); // still insurance_fund_stake base mismatch w/ spot_market
        assert_eq!(comp2.unclaimed_winnings, sweepstakes.prize_amount as u64);
        assert!(comp2.unclaimed_winnings > 0);

        insurance_fund_stake.if_base = 6; // match spot_market
        comp2
            .claim_winnings(&spot_market, &mut insurance_fund_stake)
            .unwrap();

        assert_eq!(comp2.unclaimed_winnings, 0);
        assert_eq!(comp2.unclaimed_winnings_base, 6);

        // rebased by 1 zero
        assert_eq!(
            insurance_fund_stake
                .checked_if_shares(&spot_market)
                .unwrap(),
            6
        );
    }
}
