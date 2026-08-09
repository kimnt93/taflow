use crate::error::{TaError, TaResult};

/// Fractionally-differentiated series (AFML §5.4, fixed-width window).
///
/// Weights `w_0 = 1`, `w_k = −w_{k−1}·(d−k+1)/k` truncated once
/// `|w_k| < threshold`; each output is the dot product of the weights with the
/// last `len(weights)` inputs — O(w) per bar over a ring buffer.
#[derive(Debug, Clone)]
/// Persistent Rust state or aligned output type for `FracDiff`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct FracDiff {
    weights: Box<[f64]>,
    /// Double-written ring of `2 * weights.len()` slots: each input is
    /// written at `pos` and `pos + width`, so the current window is always
    /// the contiguous slice `buffer[pos..pos + width]` (oldest to newest).
    buffer: Box<[f64]>,
    pos: usize,
    len: usize,
    value: Option<f64>,
}

impl FracDiff {
    /// Computes or updates `new` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    pub fn new(d: f64, threshold: f64) -> TaResult<Self> {
        if !(d > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "d",
                value: d.to_string(),
                reason: "must be > 0",
            });
        }
        if !(threshold > 0.0) {
            return Err(TaError::InvalidParameter {
                name: "threshold",
                value: threshold.to_string(),
                reason: "must be > 0",
            });
        }
        let mut weights = vec![1.0];
        let mut k = 1usize;
        loop {
            let wk = -weights[k - 1] * (d - k as f64 + 1.0) / k as f64;
            if wk.abs() < threshold {
                break;
            }
            weights.push(wk);
            k += 1;
        }
        let capacity = weights.len();
        Ok(Self {
            weights: weights.into_boxed_slice(),
            buffer: vec![0.0; 2 * capacity].into_boxed_slice(),
            pos: 0,
            len: 0,
            value: None,
        })
    }

    /// Computes or updates `append` through the native Rust kernel.
    ///
    /// Parameters are the typed series and configuration values in the signature.
    ///
    /// Returns the computed value, aligned history, or a validation error.
    ///
    /// The dot product accumulates newest-first (`weights[0] * latest`, then
    /// older bars), in the same order and with the same `acc += w * x`
    /// operation as the previous `VecDeque` implementation, so results are
    /// bit-identical; only the storage changed to a contiguous slice.
    pub fn append(&mut self, input: f64) -> Option<f64> {
        let width = self.weights.len();
        self.buffer[self.pos] = input;
        self.buffer[self.pos + width] = input;
        self.pos += 1;
        if self.pos == width {
            self.pos = 0;
        }
        if self.len < width {
            self.len += 1;
        }
        self.value = if self.len == width {
            let window = &self.buffer[self.pos..self.pos + width];
            let mut acc = 0.0;
            for (&w, &x) in self.weights.iter().zip(window.iter().rev()) {
                acc += w * x;
            }
            Some(acc)
        } else {
            None
        };
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
        self.pos = 0;
        self.len = 0;
        self.value = None;
    }
}
