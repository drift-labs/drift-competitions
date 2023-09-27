use anchor_lang::prelude::*;

use drift::error::ErrorCode as DriftErrorCode;

pub type CompetitionResult<T = ()> = std::result::Result<T, ErrorCode>;

#[error_code]
#[derive(PartialEq, Eq)]
pub enum ErrorCode {
    #[msg("Default")]
    Default,
    #[msg("DriftError")]
    DriftError,
    #[msg("CompetitionRoundOngoing")]
    CompetitionRoundOngoing,
    #[msg("CompetitionRoundInSettlementPhase")]
    CompetitionRoundInSettlementPhase,
    #[msg("CompetitionStatusNotActive")]
    CompetitionStatusNotActive,
    #[msg("CompetitionExpired")]
    CompetitionExpired,
    #[msg("InvalidRoundSettlementDetected")]
    InvalidRoundSettlementDetected,
    #[msg("CompetitionWinnerNotDetermined")]
    CompetitionWinnerNotDetermined,
    #[msg("CompetitorHasWrongRoundNumber")]
    CompetitorHasWrongRoundNumber,
    #[msg("CompetitorNotWinner")]
    CompetitorNotWinner,
    #[msg("InvalidStatusUpdateDetected")]
    InvalidStatusUpdateDetected,
    #[msg("InvalidIFRebase")]
    InvalidIFRebase,
    #[msg("CompetitorHasAlreadyClaimedEntry")]
    CompetitorHasAlreadyClaimedEntry,
    #[msg("CompetitorNeedsToRebaseInsuranceFundStake")]
    CompetitorNeedsToRebaseInsuranceFundStake,
    #[msg("CompetitorHasNoUnclaimedWinnings")]
    CompetitorHasNoUnclaimedWinnings,
    #[msg("CompetitionRoundNumberIssue")]
    CompetitionRoundNumberIssue,
    #[msg("CompetitorSnapshotIssue")]
    CompetitorSnapshotIssue,
    #[msg("CompetitorHasInvalidClaim")]
    CompetitorHasInvalidClaim
}

impl From<DriftErrorCode> for ErrorCode {
    fn from(_: DriftErrorCode) -> Self {
        ErrorCode::DriftError
    }
}
