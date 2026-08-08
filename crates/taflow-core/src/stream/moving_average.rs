//! Internal state dispatcher for TA-Lib-compatible moving-average types.
//!
//! This keeps consumers such as APO and PPO independent of the concrete
//! moving-average implementation while preserving each type's own warm-up.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{
    DoubleExponentialMovingAverage, ExponentialMovingAverage, KaufmanAdaptiveMovingAverage,
    MesaAdaptiveMovingAverage, SimpleMovingAverage, StreamingIndicator, TriangularMovingAverage,
    TripleExponentialAverage, TripleExponentialMovingAverage, WeightedMovingAverage,
};

pub(super) enum MovingAverageDispatcher {
    SimpleMovingAverage(SimpleMovingAverage),
    ExponentialMovingAverage(ExponentialMovingAverage),
    WeightedMovingAverage(WeightedMovingAverage),
    DoubleExponentialMovingAverage(DoubleExponentialMovingAverage),
    TripleExponentialMovingAverage(TripleExponentialMovingAverage),
    TriangularMovingAverage(TriangularMovingAverage),
    KaufmanAdaptiveMovingAverage(KaufmanAdaptiveMovingAverage),
    MesaAdaptiveMovingAverage(MesaAdaptiveMovingAverage),
    TripleExponentialAverage(TripleExponentialAverage),
}

impl MovingAverageDispatcher {
    pub(super) fn new(period: usize, ma_type: MaType) -> TaResult<Self> {
        if period == 1 {
            return Ok(Self::SimpleMovingAverage(SimpleMovingAverage::new(1)?));
        }
        Ok(match ma_type {
            MaType::SimpleMovingAverage => {
                Self::SimpleMovingAverage(SimpleMovingAverage::new(period)?)
            }
            MaType::ExponentialMovingAverage => {
                Self::ExponentialMovingAverage(ExponentialMovingAverage::new(period)?)
            }
            MaType::WeightedMovingAverage => {
                Self::WeightedMovingAverage(WeightedMovingAverage::new(period)?)
            }
            MaType::DoubleExponentialMovingAverage => {
                Self::DoubleExponentialMovingAverage(DoubleExponentialMovingAverage::new(period)?)
            }
            MaType::TripleExponentialMovingAverage => {
                Self::TripleExponentialMovingAverage(TripleExponentialMovingAverage::new(period)?)
            }
            MaType::TriangularMovingAverage => {
                Self::TriangularMovingAverage(TriangularMovingAverage::new(period)?)
            }
            MaType::KaufmanAdaptiveMovingAverage => {
                Self::KaufmanAdaptiveMovingAverage(KaufmanAdaptiveMovingAverage::new(period)?)
            }
            MaType::MesaAdaptiveMovingAverage => {
                Self::MesaAdaptiveMovingAverage(MesaAdaptiveMovingAverage::new(0.5, 0.05)?)
            }
            MaType::TripleExponentialAverage => {
                Self::TripleExponentialAverage(TripleExponentialAverage::new(period, 0.7)?)
            }
        })
    }

    pub(super) fn append(&mut self, input: f64) -> Option<f64> {
        match self {
            Self::SimpleMovingAverage(state) => state.append(input),
            Self::ExponentialMovingAverage(state) => state.append(input),
            Self::WeightedMovingAverage(state) => state.append(input),
            Self::DoubleExponentialMovingAverage(state) => state.append(input),
            Self::TripleExponentialMovingAverage(state) => state.append(input),
            Self::TriangularMovingAverage(state) => state.append(input),
            Self::KaufmanAdaptiveMovingAverage(state) => state.append(input),
            Self::MesaAdaptiveMovingAverage(state) => state.append(input).map(|value| value.mama),
            Self::TripleExponentialAverage(state) => state.append(input),
        }
    }

    /// Whether the wrapped state has produced at least one warmed value.
    #[inline]
    pub(super) fn is_warm(&self) -> bool {
        match self {
            Self::SimpleMovingAverage(state) => state.value().is_some(),
            Self::ExponentialMovingAverage(state) => state.value().is_some(),
            Self::WeightedMovingAverage(state) => state.value().is_some(),
            Self::DoubleExponentialMovingAverage(state) => state.value().is_some(),
            Self::TripleExponentialMovingAverage(state) => state.value().is_some(),
            Self::TriangularMovingAverage(state) => state.value().is_some(),
            Self::KaufmanAdaptiveMovingAverage(state) => state.value().is_some(),
            Self::MesaAdaptiveMovingAverage(state) => state.value().is_some(),
            Self::TripleExponentialAverage(state) => state.value().is_some(),
        }
    }

    /// Bulk-appends `inputs`, pushing one `f64` per bar (NaN while warming).
    ///
    /// Dispatch happens once for the whole slice instead of once per bar, and
    /// each variant's own bulk kernel runs underneath — those kernels are
    /// contractually bit-identical to their per-bar `append`, so the emitted
    /// series and the exit state match a per-bar drive exactly. MAMA has no
    /// `f64` bulk kernel and keeps the per-bar loop.
    pub(super) fn extend_slice_into(&mut self, inputs: &[f64], output: &mut Vec<f64>) {
        match self {
            Self::SimpleMovingAverage(state) => state.extend_slice_into(inputs, output),
            Self::ExponentialMovingAverage(state) => state.extend_slice_into(inputs, output),
            Self::WeightedMovingAverage(state) => state.extend_slice_into(inputs, output),
            Self::DoubleExponentialMovingAverage(state) => state.extend_slice_into(inputs, output),
            Self::TripleExponentialMovingAverage(state) => state.extend_slice_into(inputs, output),
            Self::TriangularMovingAverage(state) => state.extend_slice_into(inputs, output),
            Self::KaufmanAdaptiveMovingAverage(state) => state.extend_slice_into(inputs, output),
            Self::MesaAdaptiveMovingAverage(state) => {
                output.reserve(inputs.len());
                output.extend(
                    inputs
                        .iter()
                        .copied()
                        .map(|input| state.append(input).map_or(f64::NAN, |value| value.mama)),
                );
            }
            Self::TripleExponentialAverage(state) => state.extend_slice_into(inputs, output),
        }
    }

    /// Whether this dispatcher wraps a plain EMA state (fused bulk paths).
    #[inline]
    pub(super) fn is_ema(&self) -> bool {
        matches!(self, Self::ExponentialMovingAverage(_))
    }

    /// Mutable access to the wrapped EMA state, if this is the EMA variant.
    #[inline]
    pub(super) fn as_ema_mut(&mut self) -> Option<&mut ExponentialMovingAverage> {
        match self {
            Self::ExponentialMovingAverage(state) => Some(state),
            _ => None,
        }
    }

    /// Whether this dispatcher wraps a plain SMA state (fused bulk paths).
    ///
    /// Note that a `period == 1` dispatcher is an SMA whatever `MaType` it was
    /// built from, which is exactly the state the fused paths can drive.
    #[inline]
    pub(super) fn is_sma(&self) -> bool {
        matches!(self, Self::SimpleMovingAverage(_))
    }

    /// Mutable access to the wrapped SMA state, if this is the SMA variant.
    #[inline]
    pub(super) fn as_sma_mut(&mut self) -> Option<&mut SimpleMovingAverage> {
        match self {
            Self::SimpleMovingAverage(state) => Some(state),
            _ => None,
        }
    }

    /// Restores one SMA leg after a fused bulk loop carried its running sum in
    /// a local: rebuilds the ring from the slice tail and stores the scalar
    /// state, leaving exactly what `period` per-bar appends would have left.
    ///
    /// `inputs` must be at least `period` long and end at the bar the leg was
    /// advanced to.
    #[inline]
    pub(super) fn restore_sma_leg(state: &mut SimpleMovingAverage, inputs: &[f64], sum: f64) {
        let period = state.period();
        let window = state.window_mut();
        window.clear();
        for &input in &inputs[inputs.len() - period..] {
            window.push(input);
        }
        state.store_bulk_state(sum, Some(sum / period as f64));
    }

    pub(super) fn reset(&mut self) {
        match self {
            Self::SimpleMovingAverage(state) => state.reset(),
            Self::ExponentialMovingAverage(state) => state.reset(),
            Self::WeightedMovingAverage(state) => state.reset(),
            Self::DoubleExponentialMovingAverage(state) => state.reset(),
            Self::TripleExponentialMovingAverage(state) => state.reset(),
            Self::TriangularMovingAverage(state) => state.reset(),
            Self::KaufmanAdaptiveMovingAverage(state) => state.reset(),
            Self::MesaAdaptiveMovingAverage(state) => state.reset(),
            Self::TripleExponentialAverage(state) => state.reset(),
        }
    }
}
