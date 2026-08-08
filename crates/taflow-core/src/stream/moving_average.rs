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
