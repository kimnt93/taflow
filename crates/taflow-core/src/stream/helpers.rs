use crate::error::TaError;

/// Validate a period parameter shared by streaming indicator states.
pub(crate) fn invalid_period(name: &'static str, period: usize, minimum: usize) -> TaError {
    TaError::InvalidParameter {
        name,
        value: period.to_string(),
        reason: if minimum == 1 {
            "must be >= 1"
        } else {
            "must be >= 2"
        },
    }
}
