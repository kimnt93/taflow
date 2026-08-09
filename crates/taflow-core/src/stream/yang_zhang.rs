use super::operator_states::*;
use crate::error::{TaError, TaResult};

pub struct YangZhang {
    /// One shared ring of `[overnight, open_close, rs]` triples: the three
    /// variance components are pushed and evicted together (M4), so they need
    /// one window, one length check and one eviction index instead of three.
    window: Box<[[f64; 3]]>,
    head: usize,
    len: usize,
    sums: [f64; 3],
    means: Option<[f64; 3]>,
    timeperiod: usize,
    /// `0.34 / (1.34 + (n + 1) / (n - 1))`, constant for the state's lifetime.
    k: f64,
    previous_close: Option<f64>,
    value: Option<f64>,
}

impl YangZhang {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(timeperiod: usize) -> TaResult<Self> {
        validate_period(timeperiod)?;
        if timeperiod < 2 {
            return Err(TaError::InvalidParameter {
                name: "timeperiod",
                value: timeperiod.to_string(),
                reason: "must be >= 2 for Yang-Zhang",
            });
        }
        let n = timeperiod as f64;
        Ok(Self {
            window: vec![[0.0; 3]; timeperiod].into_boxed_slice(),
            head: 0,
            len: 0,
            sums: [0.0; 3],
            means: None,
            timeperiod,
            k: 0.34 / (1.34 + (n + 1.0) / (n - 1.0)),
            previous_close: None,
            value: None,
        })
    }

    /// Pushes one triple through the shared ring, keeping the per-component
    /// arithmetic order of the three former `RollingMean` states
    /// (`sum -= evicted`, then `sum += input`, then `sum / period`).
    #[inline]
    fn push(&mut self, sample: [f64; 3]) {
        let capacity = self.window.len();
        if self.len == capacity {
            let evicted = self.window[self.head];
            for component in 0..3 {
                self.sums[component] -= evicted[component];
            }
            self.window[self.head] = sample;
            self.head += 1;
            if self.head == capacity {
                self.head = 0;
            }
        } else {
            let mut tail = self.head + self.len;
            if tail >= capacity {
                tail -= capacity;
            }
            self.window[tail] = sample;
            self.len += 1;
        }
        for component in 0..3 {
            self.sums[component] += sample[component];
        }
        self.means = (self.len == capacity).then(|| {
            [
                self.sums[0] / self.timeperiod as f64,
                self.sums[1] / self.timeperiod as f64,
                self.sums[2] / self.timeperiod as f64,
            ]
        });
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn append(&mut self, open: f64, high: f64, low: f64, close: f64) -> Option<f64> {
        let previous_close = self.previous_close.replace(close);
        if open > 0.0 && high > 0.0 && low > 0.0 && close > 0.0 {
            if let Some(previous_close) = previous_close {
                if previous_close > 0.0 {
                    let overnight = (open / previous_close).ln().powi(2);
                    let open_close = (close / open).ln().powi(2);
                    let rs = (high / close).ln() * (high / open).ln()
                        + (low / close).ln() * (low / open).ln();
                    self.push([overnight, open_close, rs]);
                }
            }
        }
        let k = self.k;
        self.value = self
            .means
            .map(|[on, oc, rs]| (on + k * oc + (1.0 - k) * rs).max(0.0).sqrt());
        self.value
    }

    /// Computes or updates `value` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn value(&self) -> Option<f64> {
        self.value
    }

    /// Reset the persistent state and clear the latest value.
    pub fn reset(&mut self) {
        self.head = 0;
        self.len = 0;
        self.sums = [0.0; 3];
        self.means = None;
        self.previous_close = None;
        self.value = None;
    }
}
