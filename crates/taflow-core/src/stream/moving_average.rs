//! Internal state dispatcher for TA-Lib-compatible moving-average types.
//!
//! This keeps consumers such as APO and PPO independent of the concrete
//! moving-average implementation while preserving each type's own warm-up.

use crate::error::TaResult;
use crate::ma_type::MaType;

use super::{Dema, Ema, Kama, Mama, Sma, StreamingIndicator, Tema, Trima, Wma, T3};

pub(super) enum MovingAverage {
    Sma(Sma),
    Ema(Ema),
    Wma(Wma),
    Dema(Dema),
    Tema(Tema),
    Trima(Trima),
    Kama(Kama),
    Mama(Mama),
    T3(T3),
}

impl MovingAverage {
    pub(super) fn new(period: usize, ma_type: MaType) -> TaResult<Self> {
        Ok(match ma_type {
            MaType::Sma => Self::Sma(Sma::new(period)?),
            MaType::Ema => Self::Ema(Ema::new(period)?),
            MaType::Wma => Self::Wma(Wma::new(period)?),
            MaType::Dema => Self::Dema(Dema::new(period)?),
            MaType::Tema => Self::Tema(Tema::new(period)?),
            MaType::Trima => Self::Trima(Trima::new(period)?),
            MaType::Kama => Self::Kama(Kama::new(period)?),
            MaType::Mama => Self::Mama(Mama::new(0.5, 0.05)?),
            MaType::T3 => Self::T3(T3::new(period, 0.7)?),
        })
    }

    pub(super) fn append(&mut self, input: f64) -> Option<f64> {
        match self {
            Self::Sma(state) => state.append(input),
            Self::Ema(state) => state.append(input),
            Self::Wma(state) => state.append(input),
            Self::Dema(state) => state.append(input),
            Self::Tema(state) => state.append(input),
            Self::Trima(state) => state.append(input),
            Self::Kama(state) => state.append(input),
            Self::Mama(state) => state.append(input).map(|value| value.mama),
            Self::T3(state) => state.append(input),
        }
    }

    pub(super) fn reset(&mut self) {
        match self {
            Self::Sma(state) => state.reset(),
            Self::Ema(state) => state.reset(),
            Self::Wma(state) => state.reset(),
            Self::Dema(state) => state.reset(),
            Self::Tema(state) => state.reset(),
            Self::Trima(state) => state.reset(),
            Self::Kama(state) => state.reset(),
            Self::Mama(state) => state.reset(),
            Self::T3(state) => state.reset(),
        }
    }
}
