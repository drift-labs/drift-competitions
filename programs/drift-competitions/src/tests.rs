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
        sweepstakes.number_of_winners = 1;

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
        sweepstakes.number_of_winners = 1;
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
        sweepstakes.prize_randomness_max = total;

        while cnt <= ratios[0] {
            sweepstakes.prize_randomness = cnt;
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

        sweepstakes.prize_randomness = total - 1;
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

        sweepstakes.prize_randomness = total;
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
        sweepstakes.number_of_winners = 1;

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
        sweepstakes.prize_randomness_max = total;

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
            sweepstakes.prize_randomness = cnt;
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

        sweepstakes.prize_randomness = total - 1;
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

        sweepstakes.prize_randomness = total;
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
        sweepstakes.number_of_winners = 1;

        sweepstakes.sponsor_info.max_sponsor_fraction = PERCENTAGE_PRECISION_U64;

        let mut spot_market = SpotMarket::default();
        spot_market.decimals = 6;
        spot_market.insurance_fund.total_shares = 100000000000000;
        spot_market.insurance_fund.user_shares = 0;

        let vault_balance: u64 =
            (10000 * QUOTE_PRECISION) as u64 * 543532 / 2983052 + 3952730528355;
        assert_eq!(vault_balance, 3954552595151);

        let (prize_buckets, ratios) = sweepstakes
            .calculate_prize_buckets_and_ratios(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(prize_buckets, [1000000000, 5000000000, 3954552595151]);
        assert_eq!(ratios, [3961, 793, 1]);
        assert!(ratios[0] / 10 >= ratios[2]);

        let total: u128 = ratios.iter().sum();
        sweepstakes.prize_randomness_max = total; // would win max prize (if unchanged)

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
            sweepstakes.prize_randomness = cnt;
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
    use crate::state::{
        Competition, CompetitionRoundStatus, Competitor, CompetitorStatus, SponsorInfo,
    };
    use crate::utils::get_test_sample_draw;
    use drift::{
        math::{
            constants::{
                PERCENTAGE_PRECISION, PERCENTAGE_PRECISION_U64, PRICE_PRECISION_U64,
                QUOTE_PRECISION, QUOTE_PRECISION_U64,
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
        sweepstakes.number_of_winners = 1;

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
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
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
            .request_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_randomness = get_test_sample_draw(0, sweepstakes.prize_randomness_max).unwrap();
        sweepstakes.winner_randomness =
            get_test_sample_draw(1, sweepstakes.total_score_settled).unwrap();

        sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
        );
        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
        );

        assert!(sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
            .is_err());

        assert!(sweepstakes.reset_round(now).is_err());

        assert_eq!(sweepstakes.winner_randomness, 2);

        assert!(sweepstakes.settle_winner(comp1, &spot_market, now).is_err());
        sweepstakes.settle_winner(comp2, &spot_market, now).unwrap();
        assert!(sweepstakes.settle_winner(comp2, &spot_market, now).is_err()); // cannot settle twice
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );

        assert_eq!(comp2.unclaimed_winnings, sweepstakes.prize_amount as u64);

        let expected_sweepstakes = &mut Competition {
            round_number: 0,
            status: CompetitionRoundStatus::WinnerSettlementComplete,
            next_round_expiry_ts: 168000000 + 60,
            number_of_competitors: 2,
            number_of_competitors_settled: 2,
            total_score_settled: 4,
            round_duration: 60,
            winner_randomness: 2,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: 0,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes, sweepstakes);

        sweepstakes.reset_round(now).unwrap();

        let expected_sweepstakes = &mut Competition {
            number_of_winners: 1,
            number_of_winners_settled: 1,
            round_number: 1,
            status: CompetitionRoundStatus::Active,
            next_round_expiry_ts: 168000000 + 120,
            number_of_competitors: 2,
            total_score_settled: 0,
            round_duration: 60,
            winner_randomness: 0,
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
        sweepstakes.number_of_winners = 1;

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
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
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
        sweepstakes.number_of_winners = 1;

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
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
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
            .request_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_randomness = get_test_sample_draw(0, sweepstakes.prize_randomness_max).unwrap();
        sweepstakes.winner_randomness =
            get_test_sample_draw(1, sweepstakes.total_score_settled).unwrap();
        sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();
        // spot_market.insurance_fund.shares_base = 2;

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
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
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
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
        assert_eq!(sweepstakes.winner_randomness, 2);

        assert!(sweepstakes.settle_winner(comp1, &spot_market, now).is_err());

        assert_eq!(sweepstakes.prize_amount, 696202);
        assert_eq!(sweepstakes.prize_base, 1);

        sweepstakes.settle_winner(comp2, &spot_market, now).unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );

        assert_eq!(comp2.unclaimed_winnings_base, 5);
        assert_eq!(comp2.unclaimed_winnings, 69);
        assert_eq!(sweepstakes.prize_amount, 69); //rebased by 4 zeros

        assert!(sweepstakes.settle_winner(comp2, &spot_market, now).is_err()); // cannot settle twice
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );
        spot_market.insurance_fund.shares_base = 6;

        assert_eq!(comp2.competition_round_number, 1); // already on next round
        assert_eq!(comp2.unclaimed_winnings_base, 5);

        assert_eq!(comp2.unclaimed_winnings, 69);



        let expected_sweepstakes = &mut Competition {
            round_number: 0,
            status: CompetitionRoundStatus::WinnerSettlementComplete,
            next_round_expiry_ts: 168000000 + 60,
            number_of_competitors: 2,
            number_of_competitors_settled: 2,
            total_score_settled: 4,
            round_duration: 60,
            prize_base: 5,
            prize_amount: 69,
            prize_randomness: 47,
            prize_randomness_max: 95,
            outstanding_unclaimed_winnings: 69,

            winner_randomness: 2,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: PRICE_PRECISION_U64 / 2,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes, sweepstakes);
        sweepstakes.reset_round(now).unwrap();
        assert_eq!(sweepstakes.round_number, 1);
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);



        assert!(comp2.unclaimed_winnings > 0);

        let mut insurance_fund_stake = InsuranceFundStake::default();
        assert!(comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now
            )
            .is_err()); // insurance_fund_stake base mismatch w/ spot_market
        assert!(comp2.unclaimed_winnings > 0);

        insurance_fund_stake.if_base = 5; // match unclaimed_winnings_base
        assert!(comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now
            )
            .is_err()); // still insurance_fund_stake base mismatch w/ spot_market
        assert_eq!(comp2.unclaimed_winnings, 69);
        assert!(comp2.unclaimed_winnings > 0);

        insurance_fund_stake.if_base = 6; // match spot_market

        assert_eq!(comp2.unclaimed_winnings, 69);
        assert_eq!(comp2.unclaimed_winnings_base, 5);
        assert!(comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                Some(comp2.unclaimed_winnings),
                now
            )
            .is_err());

        // rebased (depsite failure). deployed contract wouldnt do this
        assert_eq!(comp2.unclaimed_winnings, 6);
        assert_eq!(comp2.unclaimed_winnings_base, 6);

        let share_to_claim_1 = comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                Some(1),
                now,
            )
            .unwrap();
        assert_eq!(share_to_claim_1, 1);
        assert_eq!(comp2.unclaimed_winnings, 5);
        assert_eq!(comp2.unclaimed_winnings_base, 6);
        let share_to_claim_2 = comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now,
            )
            .unwrap();
        assert_eq!(share_to_claim_2, 5);

        assert_eq!(comp2.unclaimed_winnings, 0);
        assert_eq!(comp2.unclaimed_winnings_base, 6);

        // rebased by 1 zero
        assert_eq!(share_to_claim_1 + share_to_claim_2, 6);
    }

    #[test]
    fn test_competition_multiple_rounds_with_no_competitors_for_long_initial_period() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.next_round_expiry_ts = now + 60;
        sweepstakes.round_duration = 60;
        sweepstakes.number_of_winners = 1;

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
            .request_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_randomness = get_test_sample_draw(0, sweepstakes.prize_randomness_max).unwrap();
        sweepstakes.winner_randomness =
            get_test_sample_draw(1, sweepstakes.total_score_settled).unwrap();
        sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
        );
        sweepstakes.settle_winner(comp1, &spot_market, now).unwrap();
        assert_eq!(sweepstakes.round_number, 0);
        assert_eq!(comp1.competition_round_number, 1);


        let expected_sweepstakes = &mut Competition {
            round_number: 0,
            status: CompetitionRoundStatus::WinnerSettlementComplete,
            next_round_expiry_ts: 168000060,
            number_of_competitors: 1,
            number_of_competitors_settled: 1,
            total_score_settled: 1,
            round_duration: 60,
            prize_base: 1,
            prize_amount: 696202,
            outstanding_unclaimed_winnings: 696202,

            prize_randomness: 478,
            prize_randomness_max: 957,
            winner_randomness: 1,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: PRICE_PRECISION_U64 / 2,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes, sweepstakes);
        sweepstakes.reset_round(now).unwrap();
        let expected_sweepstakes = &mut Competition {
            round_number: 1,
            status: CompetitionRoundStatus::Active,
            next_round_expiry_ts: 168000000 + 101 * 60,
            number_of_competitors: 1,
            total_score_settled: 0,
            round_duration: 60,
            prize_base: 1,
            prize_amount: 0,
            outstanding_unclaimed_winnings: 696202,

            prize_randomness: 0,
            prize_randomness_max: 0,
            winner_randomness: 0,
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
            .request_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_randomness = get_test_sample_draw(0, sweepstakes.prize_randomness_max).unwrap();
        sweepstakes.winner_randomness =
            get_test_sample_draw(1, sweepstakes.total_score_settled).unwrap();
        sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_randomness = 19999999999999; // inflate it crazy
        assert!(sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .is_err()); // todo: handle if balances changes between steps better
        sweepstakes.prize_randomness = sweepstakes.prize_randomness_max;

        sweepstakes
            .resolve_prize_amount(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
        );
        sweepstakes.settle_winner(comp1, &spot_market, now).unwrap();
        assert_eq!(sweepstakes.round_number, 1);
        assert_eq!(comp1.competition_round_number, 2);


        let expected_sweepstakes2 = &mut Competition {
            round_number: 1,
            status: CompetitionRoundStatus::WinnerSettlementComplete,
            next_round_expiry_ts: 168006060,
            // next_round_expiry_ts: 1113896280,

            number_of_competitors: 1,
            number_of_competitors_settled: 1,

            total_score_settled: 1,
            round_duration: 60,
            prize_base: 1,
            prize_amount: 549499999,
            outstanding_unclaimed_winnings: 550196201,

            prize_randomness: 957,
            prize_randomness_max: 957,
            winner_randomness: 1,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: PRICE_PRECISION_U64 / 2,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes2, sweepstakes);
        sweepstakes.reset_round(now).unwrap();

        expected_sweepstakes2.status = CompetitionRoundStatus::Active;
        expected_sweepstakes2.round_number = 2;
        expected_sweepstakes2.number_of_competitors_settled = 0;
        expected_sweepstakes2.total_score_settled = 0;
        expected_sweepstakes2.next_round_expiry_ts = 1113896280;
        expected_sweepstakes2.prize_amount = 0;
        expected_sweepstakes2.winner_randomness = 0;
        expected_sweepstakes2.prize_randomness = 0;
        expected_sweepstakes2.prize_randomness_max = 0;

        assert_eq!(expected_sweepstakes2, sweepstakes);
        // todo do another assert for         
        // assert_eq!(expected_sweepstakes2, sweepstakes);

    }

    #[test]
    fn test_competition_user_with_unclaimed_winnings() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.next_round_expiry_ts = now + 60;
        sweepstakes.round_duration = 60;
        sweepstakes.number_of_winners = 1;

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
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
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
            .request_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_randomness = get_test_sample_draw(0, sweepstakes.prize_randomness_max).unwrap();
        sweepstakes.winner_randomness =
            get_test_sample_draw(1, sweepstakes.total_score_settled).unwrap();

        sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();
        // spot_market.insurance_fund.shares_base = 2;

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
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
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
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
        assert_eq!(sweepstakes.winner_randomness, 2);

        assert!(sweepstakes.settle_winner(comp1, &spot_market, now).is_err());

        assert_eq!(sweepstakes.prize_amount, 696202);
        assert_eq!(sweepstakes.prize_base, 1);

        sweepstakes.settle_winner(comp2, &spot_market, now).unwrap();
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );

        assert_eq!(comp2.unclaimed_winnings_base, 5);
        assert_eq!(comp2.unclaimed_winnings, 69);
        assert_eq!(sweepstakes.prize_amount, 69); //rebased by 4 zeros

        assert!(sweepstakes.settle_winner(comp2, &spot_market, now).is_err()); // cannot settle twice
        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerSettlementComplete
        );
        spot_market.insurance_fund.shares_base = 6;

        assert_eq!(comp2.competition_round_number, 1); // already on next round
        assert_eq!(comp2.unclaimed_winnings_base, 5);

        assert_eq!(comp2.unclaimed_winnings, 69);

        let expected_sweepstakes = &mut Competition {
            round_number: 0,
            status: CompetitionRoundStatus::WinnerSettlementComplete,
            next_round_expiry_ts: 168000000 + 60,
            number_of_competitors: 2,
            number_of_competitors_settled: 2,
            total_score_settled: 4,
            round_duration: 60,
            prize_base: 5,
            prize_amount: 69,
            outstanding_unclaimed_winnings: 69,
            prize_randomness_max: 95,
            prize_randomness: 47,
            winner_randomness: 2,
            sponsor_info: SponsorInfo {
                max_sponsor_fraction: PRICE_PRECISION_U64 / 2,
                ..SponsorInfo::default()
            },
            ..Competition::default()
        };

        assert_eq!(expected_sweepstakes, sweepstakes);

        sweepstakes.reset_round(now).unwrap();

        expected_sweepstakes.status = CompetitionRoundStatus::Active;
        expected_sweepstakes.round_number += 1;
        expected_sweepstakes.number_of_competitors_settled = 0;
        expected_sweepstakes.total_score_settled = 0;
        expected_sweepstakes.next_round_expiry_ts += 60;
        expected_sweepstakes.prize_amount = 0;
        expected_sweepstakes.winner_randomness = 0;
        expected_sweepstakes.prize_randomness = 0;
        expected_sweepstakes.prize_randomness_max = 0;
        assert_eq!(expected_sweepstakes, sweepstakes);

        assert_eq!(sweepstakes.round_number, 1);
        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);

        assert!(comp2.unclaimed_winnings > 0);

        let mut insurance_fund_stake = InsuranceFundStake::default();
        assert!(comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now
            )
            .is_err()); // insurance_fund_stake base mismatch w/ spot_market
        assert!(comp2.unclaimed_winnings > 0);

        insurance_fund_stake.if_base = 5; // match unclaimed_winnings_base
        assert!(comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now
            )
            .is_err()); // still insurance_fund_stake base mismatch w/ spot_market
        assert_eq!(comp2.unclaimed_winnings, 69);
        assert!(comp2.unclaimed_winnings > 0);

        insurance_fund_stake.if_base = 6; // match spot_market

        // comp2 has unclaimed winnings, now will rerun the competition
        now += sweepstakes.round_duration as i64;

        assert!(sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
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
            .request_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_randomness = get_test_sample_draw(0, sweepstakes.prize_randomness_max).unwrap();
        sweepstakes.winner_randomness =
            get_test_sample_draw(1, sweepstakes.total_score_settled).unwrap();
        sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
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
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
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
        assert_eq!(sweepstakes.winner_randomness, 1);

        assert_eq!(sweepstakes.prize_base, 6);

        assert!(sweepstakes.settle_winner(comp2, &spot_market, now).is_err());
        sweepstakes.settle_winner(comp1, &spot_market, now).unwrap();

        assert_eq!(
            insurance_fund_stake
                .checked_if_shares(&spot_market)
                .unwrap(),
            0
        );
        let share_to_claim = comp1
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now,
            )
            .unwrap();
        assert_eq!(share_to_claim, 696202);

        let share_to_claim_2 = comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now,
            )
            .unwrap();
        assert_eq!(share_to_claim_2, 6);

        assert_eq!(comp1.competition_round_number, 2);
        assert_eq!(comp2.competition_round_number, 2);

        now += (sweepstakes.round_duration * 55 + 17) as i64;

        assert_eq!(sweepstakes.number_of_competitors, 2);
        assert_eq!(sweepstakes.number_of_competitors_settled, 2);
        sweepstakes.reset_round(now).unwrap();

        assert!(sweepstakes.settle_competitor(comp1, us, now).is_err()); // late round reset means you gotta wait til next expiry

        assert_eq!(sweepstakes.next_round_expiry_ts, 168003480); // multiple of round duration
        now += 999;
        assert_eq!(sweepstakes.round_number, 2); // 3rd round (0 indexed)

        assert!(sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
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
            .request_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        sweepstakes.prize_randomness =
            get_test_sample_draw(0, sweepstakes.prize_randomness_max).unwrap();
        sweepstakes.winner_randomness =
            get_test_sample_draw(1, sweepstakes.total_score_settled).unwrap();
        sweepstakes
            .resolve_winner_and_prize_randomness(&spot_market, vault_balance)
            .unwrap();

        assert_eq!(
            sweepstakes.status,
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
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
            CompetitionRoundStatus::WinnerAndPrizeRandomnessComplete
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
        assert_eq!(sweepstakes.winner_randomness, 6);
        sweepstakes.winner_randomness = 1; // override so comp1 wins

        assert_eq!(sweepstakes.prize_base, 6);

        assert!(sweepstakes.settle_winner(comp2, &spot_market, now).is_err());
        sweepstakes.settle_winner(comp1, &spot_market, now).unwrap();

        let share_to_claim_3 = comp1
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now,
            )
            .unwrap();
        assert!(comp2
            .claim_winnings(
                sweepstakes,
                &spot_market,
                &mut insurance_fund_stake,
                None,
                now
            )
            .is_err());
        assert_eq!(
            share_to_claim_3,
            1392410 - 696208 // delta of would-be insurance fund stake
        );
    }

    #[test]
    fn test_bonus_carry_over_logic() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.next_round_expiry_ts = now + 60;
        sweepstakes.round_duration = 60;
        sweepstakes.number_of_winners = 1;

        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);

        sweepstakes.number_of_competitors = 2;
        let comp1 = &mut Competitor::default();
        comp1.claim_entry().unwrap();

        let mut us: UserStats = UserStats::default();
        us.fees.total_fee_paid = QUOTE_PRECISION_U64 + 1;

        let last_round_score = comp1.calculate_round_score(&us).unwrap();
        assert_eq!(
            last_round_score,
            us.fees.total_fee_paid / 100 + comp1.bonus_score
        );

        sweepstakes.max_entries_per_competitor = 50;
        let last_round_score_2 = comp1.calculate_round_score(&us).unwrap();
        assert_eq!(last_round_score, last_round_score_2);

        now += 60;
        sweepstakes.settle_competitor(comp1, &us, now).unwrap();
        assert_eq!(
            comp1.bonus_score,
            (sweepstakes.max_entries_per_competitor / 2) as u64
        );
        let last_round_score_after = comp1.calculate_round_score(&us).unwrap();
        assert_eq!(last_round_score_after, comp1.bonus_score);
    }

    #[test]
    fn test_disqualified_and_requalified() {
        let mut now = 168000000;
        let sweepstakes = &mut Competition::default();

        sweepstakes.next_round_expiry_ts = now + 60;
        sweepstakes.round_duration = 60;
        sweepstakes.number_of_winners = 1;

        assert_eq!(sweepstakes.status, CompetitionRoundStatus::Active);

        sweepstakes.number_of_competitors = 4;
        let comp1 = &mut Competitor::default();
        let comp2: &mut Competitor = &mut Competitor::default();
        let comp3: &mut Competitor = &mut Competitor::default();
        let comp4: &mut Competitor = &mut Competitor::default();

        while comp2.bonus_score < 16900000 {
            comp2.claim_entry().unwrap();
            comp3.claim_entry().unwrap();
            comp3.claim_entry().unwrap();
        }

        let mut us: UserStats = UserStats::default();
        us.fees.total_fee_paid = QUOTE_PRECISION_U64 + 1;

        let us4: UserStats = UserStats::default();

        assert_eq!(comp1.status, CompetitorStatus::Active);
        comp1
            .update_status(sweepstakes, &us, CompetitorStatus::Disqualified, now)
            .unwrap();
        assert_eq!(comp1.status, CompetitorStatus::Disqualified);
        assert_eq!(sweepstakes.number_of_competitors, 3);

        comp1
            .update_status(sweepstakes, &us, CompetitorStatus::Active, now)
            .unwrap();
        assert_eq!(comp1.status, CompetitorStatus::Active);
        assert_eq!(sweepstakes.number_of_competitors, 4);

        let last_round_score = comp1.calculate_round_score(&us).unwrap();
        assert_eq!(last_round_score, 0);

        sweepstakes.max_entries_per_competitor = 500000;
        let last_round_score_2 = comp1.calculate_round_score(&us).unwrap();
        assert_eq!(last_round_score, last_round_score_2);

        now += 60;

        // can disqualify before round starts settlement
        comp1
            .update_status(sweepstakes, &us, CompetitorStatus::Disqualified, now)
            .unwrap();
        comp1
            .update_status(sweepstakes, &us, CompetitorStatus::Active, now)
            .unwrap();

        comp3
            .update_status(sweepstakes, &us, CompetitorStatus::Disqualified, now)
            .unwrap();

        sweepstakes.settle_competitor(comp1, &us, now).unwrap();

        assert_eq!(comp1.min_draw, 0);
        assert_eq!(comp1.max_draw, 0);

        assert_eq!(sweepstakes.number_of_competitors_settled, 1);
        assert_eq!(sweepstakes.total_score_settled, 0);

        // cannot disqualify comp2 once round starts settlement
        assert!(comp1
            .update_status(sweepstakes, &us, CompetitorStatus::Disqualified, now)
            .is_err());
        sweepstakes.settle_competitor(comp2, &us, now).unwrap();
        assert_eq!(comp2.min_draw, 0);
        assert_eq!(comp2.max_draw, 500000);
        // comp3 was already disqualified before start
        assert!(comp1
            .update_status(sweepstakes, &us, CompetitorStatus::Active, now)
            .is_err());
        assert_eq!(sweepstakes.number_of_competitors_settled, 2);
        assert_eq!(sweepstakes.total_score_settled, 500000);
        let score_before_comp3_settle = sweepstakes.total_score_settled;
        // no errors, just fail gracefully
        sweepstakes.settle_competitor(comp3, &us, now).unwrap();
        sweepstakes.settle_competitor(comp3, &us, now).unwrap();
        sweepstakes.settle_competitor(comp3, &us, now).unwrap();
        sweepstakes.settle_competitor(comp3, &us, now).unwrap();
        sweepstakes.settle_competitor(comp3, &us, now).unwrap();
        sweepstakes.settle_competitor(comp3, &us, now).unwrap();

        assert_eq!(comp3.min_draw, 0);
        assert_eq!(comp3.max_draw, 0);
        assert_eq!(sweepstakes.total_score_settled, score_before_comp3_settle);
        assert_eq!(sweepstakes.number_of_competitors_settled, 2);

        // cannot disqualify comp1 once settlement starts
        assert!(comp1
            .update_status(sweepstakes, &us, CompetitorStatus::Disqualified, now)
            .is_err());

        assert_eq!(
            comp2.bonus_score,
            (sweepstakes.max_entries_per_competitor / 2) as u64
        );
        assert_eq!(comp3.bonus_score, 33800000); // once flipped to active will lose this

        sweepstakes.settle_competitor(comp4, &us4, now).unwrap();
        assert_eq!(comp4.min_draw, 500000);
        assert_eq!(comp4.max_draw, 500000);

        assert_eq!(
            sweepstakes.number_of_competitors,
            sweepstakes.number_of_competitors_settled
        );

        let last_round_score_after = comp1.calculate_round_score(&us).unwrap();
        assert_eq!(last_round_score_after, comp1.bonus_score);
    }
}
