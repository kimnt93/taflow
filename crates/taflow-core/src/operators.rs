use std::collections::VecDeque;

use crate::error::{TaError, TaResult};

fn validate_period(timeperiod: usize) -> TaResult<()> {
    if timeperiod == 0 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 1",
        });
    }
    Ok(())
}

/// Delay a series by `timeperiod` bars. Warm-up values are `NaN`.
pub fn lag(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut output = vec![f64::NAN; input.len()];
    for index in timeperiod..input.len() {
        output[index] = input[index - timeperiod];
    }
    Ok(output)
}

/// Natural-log return over `timeperiod` bars. Warm-up values are `NaN`.
pub fn log_return(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut output = vec![f64::NAN; input.len()];
    for index in timeperiod..input.len() {
        output[index] = (input[index] / input[index - timeperiod]).ln();
    }
    Ok(output)
}

/// Cumulative sum of a series.
pub fn cumsum(input: &[f64]) -> Vec<f64> {
    let mut total = 0.0;
    input.iter().map(|&value| { total += value; total }).collect()
}

/// Cumulative product of a series.
pub fn cumprod(input: &[f64]) -> Vec<f64> {
    let mut total = 1.0;
    input.iter().map(|&value| { total *= value; total }).collect()
}

#[derive(Debug, Clone)]
pub struct Lag {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl Lag {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = if self.values.len() == self.timeperiod {
            let value = self.values.pop_front().expect("lag window is full");
            self.values.push_back(input);
            Some(value)
        } else {
            self.values.push_back(input);
            None
        };
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct LogReturn { lag: Lag, value: Option<f64> }

impl LogReturn {
    pub fn new(timeperiod: usize) -> TaResult<Self> { Ok(Self { lag: Lag::new(timeperiod)?, value: None }) }
    pub fn append(&mut self, input: f64) -> Option<f64> {
        self.value = self.lag.append(input).map(|previous| (input / previous).ln());
        self.value
    }
    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.lag.reset(); self.value = None; }
}

macro_rules! cumulative_operator {
    ($name:ident, $initial:expr, $operation:expr) => {
        #[derive(Debug, Clone)]
        pub struct $name { total: f64, value: Option<f64> }
        impl $name {
            pub fn new() -> Self { Self { total: $initial, value: None } }
            pub fn append(&mut self, input: f64) -> f64 {
                self.total = $operation(self.total, input);
                self.value = Some(self.total);
                self.total
            }
            pub fn value(&self) -> Option<f64> { self.value }
            pub fn reset(&mut self) { self.total = $initial; self.value = None; }
        }
        impl Default for $name { fn default() -> Self { Self::new() } }
    };
}

cumulative_operator!(Cumsum, 0.0, |total: f64, input: f64| total + input);
cumulative_operator!(Cumprod, 1.0, |total: f64, input: f64| total * input);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn batch_and_stream_match() {
        let input = vec![2.0, 4.0, 1.0, 8.0, 2.0];
        assert_eq!(lag(&input, 2).unwrap()[2..], [2.0, 4.0, 1.0]);
        assert_eq!(cumsum(&input), vec![2.0, 6.0, 7.0, 15.0, 17.0]);
        assert_eq!(cumprod(&input), vec![2.0, 8.0, 8.0, 64.0, 128.0]);
        let expected = log_return(&input, 2).unwrap();
        let mut state = LogReturn::new(2).unwrap();
        for (input, expected) in input.iter().zip(expected) {
            assert_eq!(state.append(*input).map(f64::to_bits), (!expected.is_nan()).then_some(expected.to_bits()));
        }
    }

    #[test]
    fn cumulative_states_reset() {
        let mut sum = Cumsum::new();
        let mut product = Cumprod::new();
        assert_eq!(sum.append(2.0), 2.0);
        assert_eq!(product.append(2.0), 2.0);
        sum.reset(); product.reset();
        assert_eq!(sum.append(3.0), 3.0);
        assert_eq!(product.append(3.0), 3.0);
    }
}
