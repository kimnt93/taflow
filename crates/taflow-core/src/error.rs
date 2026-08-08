use thiserror::Error;

/// Core error type for TAFlow.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum TaError {
    /// Input data is too short to compute the indicator.
    #[error("input data too short: need at least {need} elements, got {got}")]
    InsufficientData { need: usize, got: usize },

    /// A parameter value is invalid.
    #[error("invalid parameter: {name} = {value} ({reason})")]
    InvalidParameter {
        name: &'static str,
        value: String,
        reason: &'static str,
    },

    /// A required input array is missing.
    #[error("missing required input: {0}")]
    MissingInput(&'static str),

    /// Input arrays have different lengths.
    #[error("input length mismatch: expected {expected}, got {got}")]
    LengthMismatch { expected: usize, got: usize },
}

/// Convenient result type for indicator operations.
pub type TaResult<T> = Result<T, TaError>;
