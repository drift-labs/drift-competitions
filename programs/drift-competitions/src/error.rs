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
}

impl From<DriftErrorCode> for ErrorCode {
    fn from(_: DriftErrorCode) -> Self {
        ErrorCode::DriftError
    }
}
