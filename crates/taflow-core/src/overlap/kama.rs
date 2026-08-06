use crate::error::{TaError, TaResult};

/// Kaufman Adaptive Moving Average (KAMA) — O(n) sliding window
///
/// Uses C TA-Lib formulation for the serial chain:
/// prev_kama += sc² × (input[i] - prev_kama)
pub fn kama(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 1",
        });
    }
    let len = input.len();
    if timeperiod == 1 {
        return Ok(input.to_vec());
    }
    let lookback = timeperiod;
    if len <= lookback {
        return Err(TaError::InsufficientData {
            need: lookback + 1,
            got: len,
        });
    }

    let mut output = vec![0.0_f64; len];
    output[..lookback].fill(f64::NAN);

    let fast_sc = 2.0 / (2.0 + 1.0);
    let slow_sc = 2.0 / (30.0 + 1.0);
    let sc_diff = fast_sc - slow_sc;

    let mut prev_kama: f64 = input[timeperiod - 1];
    let mut volatility = 0.0_f64;
    for j in 1..=timeperiod {
        volatility += (input[j] - input[j - 1]).abs();
    }

    let direction = input[lookback] - input[0];
    let efficiency = if volatility <= direction || volatility.abs() < 1.0e-14 {
        1.0
    } else {
        (direction / volatility).abs()
    };
    let sc = efficiency.mul_add(sc_diff, slow_sc);
    prev_kama = (input[lookback] - prev_kama).mul_add(sc * sc, prev_kama);
    output[lookback] = prev_kama;

    for i in (lookback + 1)..len {
        volatility -= (input[i - timeperiod - 1] - input[i - timeperiod]).abs();
        volatility += (input[i] - input[i - 1]).abs();
        let direction = input[i] - input[i - timeperiod];
        let efficiency = if volatility <= direction || volatility.abs() < 1.0e-14 {
            1.0
        } else {
            (direction / volatility).abs()
        };
        let sc = efficiency.mul_add(sc_diff, slow_sc);
        prev_kama = (input[i] - prev_kama).mul_add(sc * sc, prev_kama);
        output[i] = prev_kama;
    }

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kama_basic() {
        let input: Vec<f64> = (1..=20).map(|x| x as f64).collect();
        let result = kama(&input, 10).unwrap();
        assert!(result[9].is_nan());
        assert!(!result[10].is_nan());
    }

    #[test]
    fn period_one_is_identity() {
        let input = vec![3.0, 1.0, 4.0, 1.0, 5.0];
        assert_eq!(kama(&input, 1).unwrap(), input);
    }
}
