use crate::error::TaResult;
use crate::stream::invalid_period;
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VolumeWeightedMovingAverageConvergenceDivergenceValue {
    pub convergence_divergence: f64,
    pub signal: f64,
    pub histogram: f64,
}

#[derive(Debug, Clone)]
struct VolumeWeightedWindow {
    period: usize,
    rows: VecDeque<(f64, f64)>,
    price_volume_sum: f64,
    volume_sum: f64,
    close_sum: f64,
}

impl VolumeWeightedWindow {
    fn new(period: usize) -> Self {
        Self {
            period,
            rows: VecDeque::with_capacity(period),
            price_volume_sum: 0.0,
            volume_sum: 0.0,
            close_sum: 0.0,
        }
    }
    fn append(&mut self, close: f64, volume: f64) -> Option<f64> {
        if self.rows.len() == self.period {
            let (old_close, old_volume) = self.rows.pop_front().expect("full window");
            self.price_volume_sum -= old_close * old_volume;
            self.volume_sum -= old_volume;
            self.close_sum -= old_close;
        }
        self.rows.push_back((close, volume));
        self.price_volume_sum += close * volume;
        self.volume_sum += volume;
        self.close_sum += close;
        (self.rows.len() == self.period).then(|| {
            if self.volume_sum > 0.0 {
                self.price_volume_sum / self.volume_sum
            } else {
                self.close_sum / self.period as f64
            }
        })
    }
    fn reset(&mut self) {
        self.rows.clear();
        self.price_volume_sum = 0.0;
        self.volume_sum = 0.0;
        self.close_sum = 0.0;
    }
}

/// MACD formed from fast/slow VWMAs and an EMA signal line.
#[derive(Debug, Clone)]
pub struct VolumeWeightedMovingAverageConvergenceDivergence {
    fast: VolumeWeightedWindow,
    slow: VolumeWeightedWindow,
    signal_period: usize,
    signal_seed: Vec<f64>,
    signal_value: Option<f64>,
    value: Option<VolumeWeightedMovingAverageConvergenceDivergenceValue>,
}

impl VolumeWeightedMovingAverageConvergenceDivergence {
    /// Create the volume-weighted oscillator; `fast` must be below `slow`.
    pub fn new(fast: usize, slow: usize, signal: usize) -> TaResult<Self> {
        if fast == 0 {
            return Err(invalid_period("fast", fast, 1));
        }
        if slow <= fast {
            return Err(invalid_period("slow", slow, fast + 1));
        }
        if signal == 0 {
            return Err(invalid_period("signal", signal, 1));
        }
        Ok(Self {
            fast: VolumeWeightedWindow::new(fast),
            slow: VolumeWeightedWindow::new(slow),
            signal_period: signal,
            signal_seed: Vec::with_capacity(signal),
            signal_value: None,
            value: None,
        })
    }

    /// Append one close/volume pair and return MACD, signal, and histogram.
    pub fn append(
        &mut self,
        close: f64,
        volume: f64,
    ) -> Option<VolumeWeightedMovingAverageConvergenceDivergenceValue> {
        let fast = self.fast.append(close, volume);
        let slow = self.slow.append(close, volume);
        let (Some(fast), Some(slow)) = (fast, slow) else {
            self.value = None;
            return None;
        };
        let convergence_divergence = fast - slow;
        let signal = if let Some(previous) = self.signal_value {
            let alpha = 2.0 / (self.signal_period as f64 + 1.0);
            let next = alpha.mul_add(convergence_divergence, (1.0 - alpha) * previous);
            self.signal_value = Some(next);
            next
        } else {
            self.signal_seed.push(convergence_divergence);
            if self.signal_seed.len() < self.signal_period {
                self.value = None;
                return None;
            }
            let seed = self.signal_seed.iter().sum::<f64>() / self.signal_period as f64;
            self.signal_value = Some(seed);
            seed
        };
        self.value = Some(VolumeWeightedMovingAverageConvergenceDivergenceValue {
            convergence_divergence,
            signal,
            histogram: convergence_divergence - signal,
        });
        self.value
    }

    /// Return the latest three oscillator values.
    pub fn value(&self) -> Option<VolumeWeightedMovingAverageConvergenceDivergenceValue> {
        self.value
    }

    /// Reset every rolling window and signal seed without reallocating.
    pub fn reset(&mut self) {
        self.fast.reset();
        self.slow.reset();
        self.signal_seed.clear();
        self.signal_value = None;
        self.value = None;
    }
}
