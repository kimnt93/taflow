//! Compatibility exports for the stream-owned exponential moving average.

pub use crate::stream::exponential_moving_average;

/// Compatibility core helper retained for moving-average selector internals.
pub fn ema_core(input: &[f64], period: usize, k: f64) -> crate::TaResult<Vec<f64>> {
    if period == 0 {
        return Err(crate::TaError::InvalidParameter { name: "timeperiod", value: "0".to_string(), reason: "must be >= 1" });
    }
    if input.len() < period {
        return Err(crate::TaError::InsufficientData { need: period, got: input.len() });
    }
    let mut output = vec![f64::NAN; input.len()];
    let mut previous = crate::simd::sum_f64(&input[..period]) / period as f64;
    output[period - 1] = previous;
    for (index, &value) in input.iter().enumerate().skip(period) {
        previous = k.mul_add(value - previous, previous);
        output[index] = previous;
    }
    Ok(output)
}
