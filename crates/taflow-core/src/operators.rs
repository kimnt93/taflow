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

/// Rolling median. Warm-up values are `NaN`; even windows average the two
/// central values.
pub fn rolling_median(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut state = RollingMedian::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
}

/// Rolling mode. Warm-up values are `NaN`; exact-value ties keep the earliest
/// value in the current window.
pub fn rolling_mode(input: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    validate_period(timeperiod)?;
    let mut state = RollingMode::new(timeperiod)?;
    Ok(input.iter().map(|&value| state.append(value).unwrap_or(f64::NAN)).collect())
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

#[derive(Debug, Clone)]
pub struct RollingMedian {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMedian {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut sorted: Vec<f64> = self.values.iter().copied().collect();
            sorted.sort_by(f64::total_cmp);
            let middle = self.timeperiod / 2;
            Some(if self.timeperiod % 2 == 1 {
                sorted[middle]
            } else {
                (sorted[middle - 1] + sorted[middle]) * 0.5
            })
        } else { None };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
}

#[derive(Debug, Clone)]
pub struct RollingMode {
    values: VecDeque<f64>,
    timeperiod: usize,
    value: Option<f64>,
}

impl RollingMode {
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        Ok(Self { values: VecDeque::with_capacity(timeperiod), timeperiod, value: None })
    }

    pub fn append(&mut self, input: f64) -> Option<f64> {
        if self.values.len() == self.timeperiod { self.values.pop_front(); }
        self.values.push_back(input);
        self.value = if self.values.len() == self.timeperiod {
            let mut best = self.values[0];
            let mut best_count = 0;
            for &candidate in &self.values {
                let count = self.values.iter().filter(|&&value| value == candidate).count();
                if count > best_count { best = candidate; best_count = count; }
            }
            Some(best)
        } else { None };
        self.value
    }

    pub fn value(&self) -> Option<f64> { self.value }
    pub fn reset(&mut self) { self.values.clear(); self.value = None; }
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

    #[test]
    fn rolling_statistics_match_batch_and_reset() {
        let input = vec![1.0, 4.0, 2.0, 2.0, 9.0, 4.0];
        let median = rolling_median(&input, 3).unwrap();
        let mode = rolling_mode(&input, 3).unwrap();
        assert!(median[0].is_nan() && median[1].is_nan());
        assert_eq!(&median[2..], &[2.0, 2.0, 2.0, 4.0]);
        assert!(mode[0].is_nan() && mode[1].is_nan());
        assert_eq!(&mode[2..], &[1.0, 2.0, 2.0, 2.0]);

        let mut state = RollingMedian::new(3).unwrap();
        for &value in &input { state.append(value); }
        state.reset();
        assert!(state.append(7.0).is_none());
    }
}
