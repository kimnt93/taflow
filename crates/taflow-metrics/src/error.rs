use thiserror::Error;

/// Errors returned while configuring or updating a metric state.
#[derive(Debug, Error, Clone, PartialEq)]
pub enum MetricError {
    /// A metric or input-converter parameter is invalid.
    #[error("invalid parameter: {name} = {value} ({reason})")]
    InvalidParameter {
        /// Public parameter name.
        name: &'static str,
        /// Supplied value formatted for diagnostics.
        value: String,
        /// Stable explanation of the accepted contract.
        reason: &'static str,
    },

    /// An observation is invalid for its selected semantic domain.
    #[error("invalid {domain} observation at position {position}: {value} ({reason})")]
    InvalidObservation {
        /// Selected input domain.
        domain: &'static str,
        /// Zero-based position among values passed to the converter.
        position: usize,
        /// Supplied value formatted for diagnostics.
        value: String,
        /// Stable explanation of the accepted contract.
        reason: &'static str,
    },

    /// Two aligned input arrays have different lengths.
    #[error("input length mismatch: expected {expected}, got {got}")]
    LengthMismatch {
        /// Length of the primary input.
        expected: usize,
        /// Length of the paired input.
        got: usize,
    },

    /// A paired converter was configured with incompatible semantic domains.
    #[error("paired input domains must match: primary is {primary}, benchmark is {benchmark}")]
    InputDomainMismatch {
        /// Primary input domain.
        primary: &'static str,
        /// Benchmark input domain.
        benchmark: &'static str,
    },
}

/// Result type for metric configuration and state updates.
pub type MetricResult<T> = Result<T, MetricError>;
