use crate::error::{TaError, TaResult};

/// Moving-average types compatible with TA-Lib `MA_Type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MaType {
    SimpleMovingAverage = 0,
    ExponentialMovingAverage = 1,
    WeightedMovingAverage = 2,
    DoubleExponentialMovingAverage = 3,
    TripleExponentialMovingAverage = 4,
    TriangularMovingAverage = 5,
    KaufmanAdaptiveMovingAverage = 6,
    MesaAdaptiveMovingAverage = 7,
    TripleExponentialAverage = 8,
}

impl TryFrom<i32> for MaType {
    type Error = TaError;

    fn try_from(value: i32) -> TaResult<Self> {
        match value {
            0 => Ok(MaType::SimpleMovingAverage),
            1 => Ok(MaType::ExponentialMovingAverage),
            2 => Ok(MaType::WeightedMovingAverage),
            3 => Ok(MaType::DoubleExponentialMovingAverage),
            4 => Ok(MaType::TripleExponentialMovingAverage),
            5 => Ok(MaType::TriangularMovingAverage),
            6 => Ok(MaType::KaufmanAdaptiveMovingAverage),
            7 => Ok(MaType::MesaAdaptiveMovingAverage),
            8 => Ok(MaType::TripleExponentialAverage),
            _ => Err(TaError::InvalidParameter {
                name: "matype",
                value: value.to_string(),
                reason: "must be 0-8",
            }),
        }
    }
}

impl MaType {
    /// Returns TA-Lib's warm-up length when this type is used by `MA`.
    pub fn lookback(self, period: usize) -> usize {
        if period == 1 {
            return 0;
        }
        match self {
            Self::SimpleMovingAverage
            | Self::ExponentialMovingAverage
            | Self::WeightedMovingAverage
            | Self::TriangularMovingAverage => period.saturating_sub(1),
            Self::DoubleExponentialMovingAverage => 2 * period.saturating_sub(1),
            Self::TripleExponentialMovingAverage => 3 * period.saturating_sub(1),
            Self::KaufmanAdaptiveMovingAverage => {
                if period == 1 {
                    0
                } else {
                    period
                }
            }
            Self::MesaAdaptiveMovingAverage => 32,
            Self::TripleExponentialAverage => 6 * period.saturating_sub(1),
        }
    }
}

/// Dispatches a `MaType` to its corresponding moving-average function.
pub fn compute_ma(input: &[f64], period: usize, ma_type: MaType) -> TaResult<Vec<f64>> {
    if period == 1 {
        return Ok(input.to_vec());
    }
    match ma_type {
        MaType::SimpleMovingAverage => crate::stream::simple_moving_average(input, period),
        MaType::ExponentialMovingAverage => {
            crate::stream::exponential_moving_average(input, period)
        }
        MaType::WeightedMovingAverage => crate::stream::weighted_moving_average(input, period),
        MaType::DoubleExponentialMovingAverage => {
            crate::stream::double_exponential_moving_average(input, period)
        }
        MaType::TripleExponentialMovingAverage => {
            let mut state = crate::stream::TripleExponentialMovingAverage::new(period)?;
            let mut output = Vec::with_capacity(input.len());
            state.extend_slice_into(input, &mut output);
            Ok(output)
        }
        MaType::TriangularMovingAverage => crate::stream::triangular_moving_average(input, period),
        MaType::KaufmanAdaptiveMovingAverage => {
            crate::stream::kaufman_adaptive_moving_average(input, period)
        }
        // MAMA and TripleExponentialAverage use fixed defaults through the
        // MA dispatcher, matching C TA-Lib ta_MA.c:
        //   MAMA: fastlimit=0.5, slowlimit=0.05 (period is ignored)
        //   TripleExponentialAverage: vfactor=0.7 (period is forwarded)
        MaType::MesaAdaptiveMovingAverage => {
            let (mama, _fama) = crate::stream::mesa_adaptive_moving_average(input, 0.5, 0.05)?;
            Ok(mama)
        }
        MaType::TripleExponentialAverage => {
            crate::stream::triple_exponential_average(input, period, 0.7)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn period_one_is_identity_for_every_dispatched_type() {
        let input = [1.0, 3.0, 2.0, 8.0];
        for code in 0..=8 {
            let ma_type = MaType::try_from(code).unwrap();
            assert_eq!(ma_type.lookback(1), 0);
            assert_eq!(compute_ma(&input, 1, ma_type).unwrap(), input);
        }
    }
}
