use candid::CandidType;
use thiserror::Error;

#[derive(Error, Debug, CandidType)]
pub enum AppError {
    #[error("{0} not found")]
    NotFound(String),
}
