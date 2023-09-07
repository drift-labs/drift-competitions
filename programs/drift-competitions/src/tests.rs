#[cfg(test)]
mod competition_fcn {
    use crate::state::{Competition, CompetitionRoundStatus, Competitor};
    use anchor_lang::prelude::Pubkey;
    use drift::state::user::UserStats;

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
        comp2.claim_entry().unwrap();
        comp2.claim_entry().unwrap();

        let us: UserStats = UserStats::default();

        assert!(sweepstakes.settle_competitor(comp1, us, now).is_err());
        assert!(sweepstakes.settle_competitor(comp2, us, now).is_err());

        now += 60;

        assert!(sweepstakes.reset_round().is_err());
        assert!(sweepstakes.resolve_round().is_err());

        sweepstakes.settle_competitor(comp1, us, now).unwrap();
        sweepstakes.settle_competitor(comp2, us, now).unwrap();

        assert_eq!(comp1.min_draw, 0);
        assert_eq!(comp1.max_draw, 1);

        assert_eq!(comp2.min_draw, 1);
        assert_eq!(comp2.max_draw, 4);

        sweepstakes.resolve_round().unwrap();
        assert!(sweepstakes.reset_round().is_err());

        assert_eq!(sweepstakes.winning_draw, 2);

        assert!(sweepstakes.settle_winner(comp1).is_err());
        sweepstakes.settle_winner(comp2).unwrap();

        sweepstakes.reset_round().unwrap();
        assert_eq!(sweepstakes.round_number, 1);
    }
}
