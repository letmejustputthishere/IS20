use candid::{CandidType, Deserialize};
use ic_helpers::tokens::Tokens128;
use thiserror::Error;

use crate::types::Timestamp;

#[derive(CandidType, Debug, PartialEq, Deserialize, Error)]
pub enum TxError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Amount too small")]
    AmountTooSmall,
    #[error("Fee exceeded limit {fee_limit}")]
    FeeExceededLimit { fee_limit: Tokens128 },
    #[error("Already actioned")]
    AlreadyActioned,
    #[error("Transaction does not exist")]
    TransactionDoesNotExist,
    #[error("Bad fee {expected_fee}")]
    BadFee { expected_fee: Tokens128 },
    #[error("Insufficient funds : {balance}")]
    InsufficientFunds { balance: Tokens128 },
    #[error("Transaction is too old : {allowed_window_nanos}")]
    TooOld { allowed_window_nanos: u64 },
    #[error("Transaction is created in the future {ledger_time}")]
    CreatedInFuture { ledger_time: u64 },
    #[error("Transaction is duplicate of {duplicate_of}")]
    Duplicate { duplicate_of: u64 },
    #[error("Self transfer")]
    SelfTransfer,
    #[error("Amount overflow")]
    AmountOverflow,
    #[error("Account is not found")]
    AccountNotFound,
    #[error("{message}")]
    GenericError { message: String },
    #[error("Claim not Allowed")]
    ClaimNotAllowed,
    #[error("Temporary unavailable")]
    TemporaryUnavailable,
}

#[derive(CandidType, Debug, PartialEq, Deserialize)]
pub enum TransferError {
    BadFee { expected_fee: Tokens128 },
    BadBurn { min_burn_amount: Tokens128 },
    InsufficientFunds { balance: Tokens128 },
    TooOld,
    CreatedInFuture { ledger_time: Timestamp },
    Duplicate { duplicate_of: u128 },
    TemporarilyUnavailable,
    GenericError { error_code: u128, message: String },
}

impl From<TxError> for TransferError {
    fn from(err: TxError) -> Self {
        match err {
            TxError::FeeExceededLimit { fee_limit } => Self::BadFee {
                expected_fee: fee_limit,
            },
            TxError::BadFee { expected_fee } => Self::BadFee { expected_fee },
            TxError::InsufficientFunds { balance } => Self::InsufficientFunds { balance },
            TxError::TooOld { .. } => Self::TooOld,
            TxError::CreatedInFuture { ledger_time } => Self::CreatedInFuture { ledger_time },
            TxError::Duplicate { duplicate_of } => Self::Duplicate {
                duplicate_of: duplicate_of as u128,
            },
            TxError::TemporaryUnavailable => Self::TemporarilyUnavailable,
            _ => TransferError::GenericError {
                error_code: 500,
                message: format!("{err}"),
            },
        }
    }
}
