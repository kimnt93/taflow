use crate::error::{TaError, TaResult};

pub(crate) fn validate_ohlc_len(
    len: usize,
    high: &[f64],
    low: &[f64],
    close: &[f64],
) -> TaResult<()> {
    if len != high.len() || len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: high.len().min(low.len()).min(close.len()),
        });
    }
    Ok(())
}
