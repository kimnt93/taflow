//! Stateful Stochastic Relative Strength Index.
//!
//! STOCHRSI pipelines each warmed Wilder RSI value through the persistent
//! Fast Stochastic state, preserving adaptive-MA seed and rounding semantics.

use crate::error::TaResult;
use crate::ma_type::MaType;

use crate::indicators::{FastStochasticOscillator, RelativeStrengthIndex};
use crate::stream::StreamingIndicator;

/// One aligned stochastic-RSI fast %K and fast %D observation.
#[derive(Debug, Clone, Copy, PartialEq)]
/// Persistent Rust state or aligned output type for `StochasticRelativeStrengthIndexValue`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct StochasticRelativeStrengthIndexValue {
    pub fastk: f64,
    pub fastd: f64,
}

/// Incremental STOCHRSI state.
/// Persistent Rust state or aligned output type for `StochasticRelativeStrengthIndex`.
///
/// The state consumes chronological inputs causally, preserves warm-up
/// values, and exposes the current result through its public API.
pub struct StochasticRelativeStrengthIndex {
    rsi: RelativeStrengthIndex,
    stochastic: FastStochasticOscillator,
    value: Option<StochasticRelativeStrengthIndexValue>,
}

impl StochasticRelativeStrengthIndex {
    /// Creates STOCHRSI with a selectable fast-%D moving-average type.
    pub fn new(
        timeperiod: usize,
        fastk_period: usize,
        fastd_period: usize,
        fastd_matype: MaType,
    ) -> TaResult<Self> {
        Ok(Self {
            rsi: RelativeStrengthIndex::new(timeperiod)?,
            stochastic: FastStochasticOscillator::new(fastk_period, fastd_period, fastd_matype)?,
            value: None,
        })
    }

    /// Appends one close value.
    pub fn append(&mut self, input: f64) -> Option<StochasticRelativeStrengthIndexValue> {
        self.value = self.rsi.append(input).and_then(|rsi| {
            self.stochastic.append(rsi, rsi, rsi).map(|value| {
                StochasticRelativeStrengthIndexValue {
                    fastk: value.fastk,
                    fastd: value.fastd,
                }
            })
        });
        self.value
    }

    /// Bulk kernel: the Wilder RSI recurrence runs per bar (it is a two-FLOP
    /// serial recurrence, not the bottleneck), the warmed RSI values are then
    /// handed to [`FastStochasticOscillator::extend_slices_into`], whose vHGW
    /// extrema pass removes the O(n * fastk_period) work. Outputs and post-run
    /// state are bit-identical to per-bar [`Self::append`]; warm-up bars are
    /// NaN.
    pub fn extend_slices_into(
        &mut self,
        inputs: &[f64],
        fastk_out: &mut Vec<f64>,
        fastd_out: &mut Vec<f64>,
    ) {
        fastk_out.reserve(inputs.len());
        fastd_out.reserve(inputs.len());
        let mut warmed = Vec::with_capacity(inputs.len());
        for &input in inputs {
            if let Some(rsi) = self.rsi.append(input) {
                warmed.push(rsi);
            }
        }
        // RSI warm-up is a strict prefix, so the NaN bars are exactly the
        // leading `inputs.len() - warmed.len()` positions.
        for _ in 0..(inputs.len() - warmed.len()) {
            fastk_out.push(f64::NAN);
            fastd_out.push(f64::NAN);
        }
        self.stochastic
            .extend_slices_into(&warmed, &warmed, &warmed, fastk_out, fastd_out)
            .expect("identical slice lengths");
        self.value = self
            .stochastic
            .value()
            .map(|value| StochasticRelativeStrengthIndexValue {
                fastk: value.fastk,
                fastd: value.fastd,
            });
    }

    /// Returns the latest warmed output.
    pub fn value(&self) -> Option<StochasticRelativeStrengthIndexValue> {
        self.value
    }

    /// Restores the post-construction state.
    pub fn reset(&mut self) {
        self.rsi.reset();
        self.stochastic.reset();
        self.value = None;
    }
}
