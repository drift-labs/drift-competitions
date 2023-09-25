#[cfg(test)]

mod competition_helpers {
    use crate::state::Competition;
    use drift::{
        math::{
            constants::{PERCENTAGE_PRECISION_U64, QUOTE_PRECISION},
            insurance::if_shares_to_vault_amount,
        },
        state::spot_market::SpotMarket,
    };
    #[test]
    pub fn test_calculate_next_round_expiry_ts() {
        let mut now = 1695330779;
        let sweepstakes = &mut Competition::default();
        sweepstakes.round_duration = 604800;

        let first_expiry = 1695650400;

        sweepstakes.next_round_expiry_ts = first_expiry;

        let expiry_ts = sweepstakes.calculate_next_round_expiry_ts(now).unwrap();
        let expected_ts = first_expiry;

        assert_eq!(expiry_ts, expected_ts);

        while now < expiry_ts {
            assert_eq!(
                sweepstakes.calculate_next_round_expiry_ts(now).unwrap(),
                expected_ts
            );
            now += 1;
        }

        assert_eq!(now, expected_ts);

        assert_eq!(
            sweepstakes.calculate_next_round_expiry_ts(now).unwrap(),
            expected_ts + sweepstakes.round_duration as i64
        );

        while now < expiry_ts * 6 + 191 {
            assert!(
                sweepstakes.calculate_next_round_expiry_ts(now).unwrap()
                    >= expected_ts + sweepstakes.round_duration as i64
            );
            assert!(sweepstakes.calculate_next_round_expiry_ts(now).unwrap() >= now);
            assert_eq!(
                (sweepstakes.calculate_next_round_expiry_ts(now).unwrap() - first_expiry)
                    % sweepstakes.round_duration as i64,
                0
            );
            now += 456333;
        }
    }

    #[test]
    pub fn test_prize_odds() {
        let sweepstakes = &mut Competition::default();
        sweepstakes.sponsor_info.max_sponsor_fraction = PERCENTAGE_PRECISION_U64;

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 100;
        spot_market.insurance_fund.user_shares = 0;

        // 10k max
        let vault_balance: u64 = (10000 * QUOTE_PRECISION) as u64;
        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 10000000000]);
        assert_eq!(ratios, [16, 4, 1]);
        assert!(ratios[0] / 10 >= ratios[2]);

        // 10.1k max
        let vault_balance: u64 = (10100 * QUOTE_PRECISION + 35235) as u64;
        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 10100035235]);
        assert_eq!(ratios, [17, 4, 1]);
        assert!(ratios[0] / 10 >= ratios[2]);

        // 100k max
        let vault_balance: u64 = (100000 * QUOTE_PRECISION) as u64;
        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 100000000000]);
        assert_eq!(ratios, [106, 22, 1]);
        assert!(ratios[0] / 100 >= ratios[2]);

        let total = ratios.iter().sum();
        let mut cnt = 0;
        sweepstakes.prize_draw_max = total;

        while cnt <= ratios[0] {
            sweepstakes.prize_draw = cnt;
            let prize_shares = sweepstakes
                .calculate_prize_amount(&spot_market, vault_balance)
                .unwrap();
            let prize_quote = if_shares_to_vault_amount(
                prize_shares,
                spot_market.insurance_fund.total_shares,
                vault_balance,
            )
            .unwrap() as u128;
            assert_eq!(prize_quote, prize_buckets[0]);
            cnt += 1;
        }

        sweepstakes.prize_draw = total - 1;
        let prize_shares = sweepstakes
            .calculate_prize_amount(&spot_market, vault_balance)
            .unwrap();
        let prize_quote = if_shares_to_vault_amount(
            prize_shares,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )
        .unwrap() as u128;
        assert_eq!(prize_shares, spot_market.insurance_fund.total_shares / 20);

        assert_eq!(prize_quote, prize_buckets[1]);

        sweepstakes.prize_draw = total;
        let prize_shares = sweepstakes
            .calculate_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(prize_shares, spot_market.insurance_fund.total_shares);
        let prize_quote = if_shares_to_vault_amount(
            prize_shares,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )
        .unwrap() as u128;
        assert_eq!(prize_quote, prize_buckets[2]);
    }

    #[test]
    pub fn test_prize_odds_insurance_odd_lot() {
        let sweepstakes = &mut Competition::default();
        sweepstakes.sponsor_info.max_sponsor_fraction = PERCENTAGE_PRECISION_U64;

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 100;
        spot_market.insurance_fund.user_shares = 0;

        // 10k max
        let vault_balance: u64 =
            (10000 * QUOTE_PRECISION) as u64 * 543532 / 2983052 + 3952730528355;
        assert_eq!(vault_balance, 3954552595151);

        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 3954552595151]);
        assert_eq!(ratios, [3961, 793, 1]);
        assert!(ratios[0] / 10 >= ratios[2]);

        // 10.1k max
        let vault_balance: u64 = (10100 * QUOTE_PRECISION + 35235) as u64;
        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 10100035235]);
        assert_eq!(ratios, [17, 4, 1]);
        assert!(ratios[0] / 10 >= ratios[2]);

        let total = ratios.iter().sum();
        let mut cnt = 0;
        sweepstakes.prize_draw_max = total;

        let prize_shares = sweepstakes
            .calculate_prize_amount(&spot_market, vault_balance)
            .unwrap();
        let prize_quote = if_shares_to_vault_amount(
            prize_shares,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )
        .unwrap() as u128;
        assert!(prize_quote < prize_buckets[0]);
        assert_eq!(prize_quote, 909003171);

        // higher total shares allows for more grainularity
        spot_market.insurance_fund.total_shares *= 1000000;
        let prize_shares = sweepstakes
            .calculate_prize_amount(&spot_market, vault_balance)
            .unwrap();
        let prize_quote = if_shares_to_vault_amount(
            prize_shares,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )
        .unwrap() as u128;
        assert!(prize_quote <= prize_buckets[0]);
        assert_eq!(prize_quote, 999999943);

        while cnt <= ratios[0] {
            sweepstakes.prize_draw = cnt;
            let prize_shares = sweepstakes
                .calculate_prize_amount(&spot_market, vault_balance)
                .unwrap();
            let prize_quote = if_shares_to_vault_amount(
                prize_shares,
                spot_market.insurance_fund.total_shares,
                vault_balance,
            )
            .unwrap() as u128;
            assert!(prize_quote < prize_buckets[0]);
            assert_eq!(prize_quote, 999999943);

            cnt += 1;
        }

        sweepstakes.prize_draw = total - 1;
        let prize_shares = sweepstakes
            .calculate_prize_amount(&spot_market, vault_balance)
            .unwrap();
        let prize_quote = if_shares_to_vault_amount(
            prize_shares,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )
        .unwrap() as u128;
        assert_eq!(prize_shares, 49504777);

        assert_eq!(prize_quote, prize_buckets[1] - 80); // slightly below (rounding in favor)

        sweepstakes.prize_draw = total;
        let prize_shares = sweepstakes
            .calculate_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(prize_shares, spot_market.insurance_fund.total_shares);
        let prize_quote = if_shares_to_vault_amount(
            prize_shares,
            spot_market.insurance_fund.total_shares,
            vault_balance,
        )
        .unwrap() as u128;
        assert_eq!(prize_quote, prize_buckets[2]); // no rounding since 100%
    }

    #[test]
    pub fn test_prize_odds_changing_insurance_fund() {
        let sweepstakes = &mut Competition::default();
        sweepstakes.sponsor_info.max_sponsor_fraction = PERCENTAGE_PRECISION_U64;

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 100000000000000;
        spot_market.insurance_fund.user_shares = 0;

        let mut vault_balance: u64 =
            (10000 * QUOTE_PRECISION) as u64 * 543532 / 2983052 + 3952730528355;
        assert_eq!(vault_balance, 3954552595151);

        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 3954552595151]);
        assert_eq!(ratios, [3961, 793, 1]);
        assert!(ratios[0] / 10 >= ratios[2]);

        let total: u128 = ratios.iter().sum();
        sweepstakes.prize_draw_max = total; // would win max prize (if unchanged)

        let if_deltas = [
            0,
            -1,
            1,
            -((QUOTE_PRECISION / 33_u128) as i64),
            (QUOTE_PRECISION / 33_u128) as i64,
            QUOTE_PRECISION as i64,
            -(QUOTE_PRECISION as i64),
            959898770,
            -4869334,
            -(vault_balance as i64) + 1,
            -(vault_balance as i64) / 2,
            (vault_balance as i64),
        ];

        let mut cnt = 0;

        let mut min_prize_times = [0, 0, 0];
        while cnt <= total {
            sweepstakes.prize_draw = cnt;
            for (i, &if_delta) in if_deltas.iter().enumerate() {
                println!("{} {}", i, if_delta);
                let vv = (vault_balance as i64 + if_delta) as u64;
                let prize_shares = sweepstakes
                    .calculate_prize_amount(&spot_market, vv)
                    .unwrap();
                let prize_quote = if_shares_to_vault_amount(
                    prize_shares,
                    spot_market.insurance_fund.total_shares,
                    vv,
                )
                .unwrap() as u128;

                if prize_quote <= prize_buckets[0] {
                    min_prize_times[0] += 1;
                } else if prize_quote <= prize_buckets[1] {
                    min_prize_times[1] += 1;
                } else {
                    min_prize_times[2] += 1;
                }
            }
            cnt += 1;
        }
        assert_eq!(min_prize_times, [49141, 7923, 8]); // only when cnt = max prize draw
        assert!(min_prize_times[2] < if_deltas.len());
    }
}

mod competition_fcn {
    use crate::state::{Competition, CompetitionRoundStatus, Competitor, SponsorInfo};
    use drift::{
        math::{
            constants::{
                PERCENTAGE_PRECISION, PERCENTAGE_PRECISION_U64, PRICE_PRECISION_U64,
                QUOTE_PRECISION,
            },
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

        sweepstakes.next_round_expiry_ts = now + 60;
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

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 1100;
        spot_market.insurance_fund.user_shares = 1000;

        let vault_balance: u64 = (1580000 * QUOTE_PRECISION) as u64;

        assert!(sweepstakes.reset_round(now).is_err());
        assert!(sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .is_err());

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
        sweepstakes
            .request_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();

        sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
        );
        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
        );

        assert!(sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .is_err());

        assert!(sweepstakes.reset_round(now).is_err());

        assert_eq!(sweepstakes.winning_draw, 2);

        assert!(sweepstakes.settle_winner(comp1, &spot_market).is_err());
        sweepstakes.settle_winner(comp2, &spot_market).unwrap();
        assert!(sweepstakes.settle_winner(comp2, &spot_market).is_err()); // cannot settle twice
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );

        assert_eq!(comp2.unclaimed_winnings, sweepstakes.prize_amount as u64);

        sweepstakes.reset_round(now).unwrap();

        let expected_sweepstakes = &mut Competition {
            round_number: 1,
            status: CompetitionRoundStatus::Active,
            next_round_expiry_ts: 168000000 + 120,
            number_of_competitors: 2,
            total_score_settled: 0,
            round_duration: 60,
            winning_draw: 2,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: 0,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes, sweepstakes);
    }

    #[test]
    fn test_competition_expiry() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.next_round_expiry_ts = now + 60;
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

        assert!(sweepstakes.reset_round(now).is_err());
        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 1100;
        spot_market.insurance_fund.user_shares = 0;

        let vault_balance: u64 = (1 * QUOTE_PRECISION) as u64;
        assert!(sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .is_err());

        let us: &UserStats = &UserStats::default();

        assert!(sweepstakes.settle_competitor(comp1, us, now).is_err());
        assert!(sweepstakes.settle_competitor(comp2, us, now).is_err());

        sweepstakes.expire(now).unwrap();

        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Expired);
    }

    #[test]
    fn test_competition_prize_rebases() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.next_round_expiry_ts = now + 60;
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

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 1100 * PERCENTAGE_PRECISION;
        spot_market.insurance_fund.user_shares = 1000 * PERCENTAGE_PRECISION;
        spot_market.insurance_fund.shares_base = 1;
        let vault_balance: u64 = (1580000 * QUOTE_PRECISION) as u64;

        assert!(sweepstakes.reset_round(now).is_err());
        assert!(sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .is_err());

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
        sweepstakes
            .request_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();
        sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();
        // spot_market.insurance_fund.shares_base = 2;

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
        );

        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 71818181818]);
        assert_eq!(ratios, [78, 16, 1]);

        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
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

        spot_market.insurance_fund.shares_base = 4;
        assert_eq!(sweepstakes.prize_amount, 696202);

        assert!(sweepstakes.reset_round(now).is_err());
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

        sweepstakes.reset_round(now).unwrap();
        assert_eq!(sweepstakes.round_number, 1);
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);

        let expected_sweepstakes = &mut Competition {
            round_number: 1,
            status: CompetitionRoundStatus::Active,
            next_round_expiry_ts: 168000000 + 120,
            number_of_competitors: 2,
            total_score_settled: 0,
            round_duration: 60,
            prize_base: 5,
            prize_amount: 69,
            prize_draw: 47,
            prize_draw_max: 95,
            winning_draw: 2,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: PRICE_PRECISION_U64 / 2,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes, sweepstakes);

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

    #[test]
    fn test_competition_multiple_rounds_with_no_competitors_for_long_initial_period() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.next_round_expiry_ts = now + 60;
        sweepstakes.round_duration = 60;
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);
        sweepstakes.sponsor_info.max_sponsor_fraction = PERCENTAGE_PRECISION_U64 / 2;
        sweepstakes.number_of_competitors = 2;
        assert!(sweepstakes.reset_round(now).is_err());

        now += 60 * 100; // 100 rounds have passed with no competitors

        let comp1 = &mut Competitor::default();
        sweepstakes.number_of_competitors = 1;
        comp1.claim_entry().unwrap();
        assert!(sweepstakes.reset_round(now).is_err());

        let us: &UserStats = &UserStats::default();

        sweepstakes.settle_competitor(comp1, us, now).unwrap();

        assert!(sweepstakes.reset_round(now).is_err());

        assert_eq!(comp1.min_draw, 0);
        assert_eq!(comp1.max_draw, 1);

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 1100 * PERCENTAGE_PRECISION;
        spot_market.insurance_fund.user_shares = 1 * PERCENTAGE_PRECISION;
        spot_market.insurance_fund.shares_base = 1;
        let vault_balance: u64 = (1580000 * QUOTE_PRECISION) as u64;
        sweepstakes
            .request_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();
        sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
        );
        sweepstakes.settle_winner(comp1, &spot_market).unwrap();
        assert_eq!(sweepstakes.round_number, 0);
        assert_eq!(comp1.competition_round_number, 1);

        sweepstakes.reset_round(now).unwrap();

        let expected_sweepstakes = &mut Competition {
            round_number: 1,
            status: CompetitionRoundStatus::Active,
            next_round_expiry_ts: 168000000 + 101 * 60,
            number_of_competitors: 1,
            total_score_settled: 0,
            round_duration: 60,
            prize_base: 1,
            prize_amount: 696202,
            prize_draw: 478,
            prize_draw_max: 957,
            winning_draw: 1,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: PRICE_PRECISION_U64 / 2,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes, sweepstakes);

        // do another round
        now += 945890235;
        comp1.bonus_score += 1;
        comp1.unclaimed_winnings = 0;
        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        sweepstakes
            .request_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();
        sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_draw = 19999999999999; // inflate it crazy
        assert!(sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .is_err()); // todo: handle if balances changes between steps better
        sweepstakes.prize_draw = sweepstakes.prize_draw_max;

        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
        );
        sweepstakes.settle_winner(comp1, &spot_market).unwrap();
        assert_eq!(sweepstakes.round_number, 1);
        assert_eq!(comp1.competition_round_number, 2);

        sweepstakes.reset_round(now).unwrap();

        let expected_sweepstakes2 = &mut Competition {
            round_number: 2,
            status: CompetitionRoundStatus::Active,
            next_round_expiry_ts: 1113896280,
            number_of_competitors: 1,
            total_score_settled: 0,
            round_duration: 60,
            prize_base: 1,
            prize_amount: 549499999,
            prize_draw: 957,
            prize_draw_max: 957,
            winning_draw: 1,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: PRICE_PRECISION_U64 / 2,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes2, sweepstakes);
    }

    #[test]
    fn test_competition_user_with_unclaimed_winnings() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.next_round_expiry_ts = now + 60;
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

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 1100 * PERCENTAGE_PRECISION;
        spot_market.insurance_fund.user_shares = 1000 * PERCENTAGE_PRECISION;
        spot_market.insurance_fund.shares_base = 1;
        let vault_balance: u64 = (1580000 * QUOTE_PRECISION) as u64;

        assert!(sweepstakes.reset_round(now).is_err());
        assert!(sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .is_err());

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
        sweepstakes
            .request_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();

        sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();
        // spot_market.insurance_fund.shares_base = 2;

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
        );

        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 71818181818]);
        assert_eq!(ratios, [78, 16, 1]);

        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
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

        spot_market.insurance_fund.shares_base = 4;
        assert_eq!(sweepstakes.prize_amount, 696202);

        assert!(sweepstakes.reset_round(now).is_err());
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

        sweepstakes.reset_round(now).unwrap();
        assert_eq!(sweepstakes.round_number, 1);
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);

        let expected_sweepstakes = &mut Competition {
            round_number: 1,
            status: CompetitionRoundStatus::Active,
            next_round_expiry_ts: 168000000 + 120,
            number_of_competitors: 2,
            total_score_settled: 0,
            round_duration: 60,
            prize_base: 5,
            prize_amount: 69,
            prize_draw_max: 95,
            prize_draw: 47,
            winning_draw: 2,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: PRICE_PRECISION_U64 / 2,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes, sweepstakes);

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

        // comp2 has unclaimed winnings, now will rerun the competition
        now += sweepstakes.round_duration as i64;

        assert!(sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .is_err());

        comp1.bonus_score = 1;
        comp2.bonus_score = 10;

        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 1);
        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 1); // resettle a competitor doesnt increment
        assert_eq!(sweepstakes.total_score_settled, 1);

        sweepstakes.settle_competitor(comp2, us, now).unwrap();
        assert_eq!(sweepstakes.total_score_settled, 1); // comp2 skipped since they already won and didnt claim
        assert!(sweepstakes.number_of_competitors_settled == 2);
        sweepstakes
            .request_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();
        sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
        );

        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 71818181818]);
        assert_eq!(ratios, [78, 16, 1]);

        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
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

        assert_eq!(sweepstakes.prize_amount, 696202);

        assert!(sweepstakes.reset_round(now).is_err());
        assert_eq!(sweepstakes.winning_draw, 1);

        assert_eq!(sweepstakes.prize_base, 6);

        assert!(sweepstakes.settle_winner(comp2, &spot_market).is_err());
        sweepstakes.settle_winner(comp1, &spot_market).unwrap();

        assert_eq!(
            insurance_fund_stake
                .checked_if_shares(&spot_market)
                .unwrap(),
            0
        );
        comp1
            .claim_winnings(&spot_market, &mut insurance_fund_stake)
            .unwrap();
        assert_eq!(
            insurance_fund_stake
                .checked_if_shares(&spot_market)
                .unwrap(),
            696202
        );

        comp2
            .claim_winnings(&spot_market, &mut insurance_fund_stake)
            .unwrap();
        assert_eq!(
            insurance_fund_stake
                .checked_if_shares(&spot_market)
                .unwrap(),
            696208
        );

        assert_eq!(comp1.competition_round_number, 2);
        assert_eq!(comp2.competition_round_number, 2);

        // both have claimed, now try running another round (but resetting it late)
        now += (sweepstakes.round_duration * 55 + 17) as i64;

        sweepstakes.reset_round(now).unwrap();

        assert!(sweepstakes.settle_competitor(comp1, us, now).is_err()); // late round reset means you gotta wait til next expiry

        assert_eq!(sweepstakes.next_round_expiry_ts, 168003480); // multiple of round duration
        now += 999;
        assert_eq!(sweepstakes.round_number, 2); // 3rd round (0 indexed)

        assert!(sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .is_err());

        comp1.bonus_score = 1;
        comp2.bonus_score = 10;

        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 1);
        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        assert!(sweepstakes.number_of_competitors_settled == 1); // resettle a competitor doesnt increment
        assert_eq!(sweepstakes.total_score_settled, 1);

        sweepstakes.settle_competitor(comp2, us, now).unwrap();
        assert_eq!(sweepstakes.total_score_settled, 11);
        assert_eq!(sweepstakes.number_of_competitors_settled, 2);
        sweepstakes
            .request_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();
        sweepstakes
            .resolve_winner_and_prize_draw(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
        );

        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 71818181818]);
        assert_eq!(ratios, [78, 16, 1]);

        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeDrawComplete
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

        assert_eq!(sweepstakes.prize_amount, 696202);

        assert!(sweepstakes.reset_round(now).is_err());
        assert_eq!(sweepstakes.winning_draw, 6);
        sweepstakes.winning_draw = 1; // override so comp1 wins

        assert_eq!(sweepstakes.prize_base, 6);

        assert!(sweepstakes.settle_winner(comp2, &spot_market).is_err());
        sweepstakes.settle_winner(comp1, &spot_market).unwrap();

        assert_eq!(
            insurance_fund_stake
                .checked_if_shares(&spot_market)
                .unwrap(),
            696208
        );
        comp1
            .claim_winnings(&spot_market, &mut insurance_fund_stake)
            .unwrap();
        assert!(comp2
            .claim_winnings(&spot_market, &mut insurance_fund_stake)
            .is_err());
        assert_eq!(
            insurance_fund_stake
                .checked_if_shares(&spot_market)
                .unwrap(),
            1392410
        );
    }
}
