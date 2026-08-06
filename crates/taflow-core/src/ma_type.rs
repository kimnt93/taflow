use crate::error::{TaError, TaResult};

/// 移动平均类型枚举，与 TA-Lib 的 MA_Type 完全兼容
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum MaType {
    Sma = 0,
    Ema = 1,
    Wma = 2,
    Dema = 3,
    Tema = 4,
    Trima = 5,
    Kama = 6,
    Mama = 7,
    T3 = 8,
}

impl TryFrom<i32> for MaType {
    type Error = TaError;

    fn try_from(value: i32) -> TaResult<Self> {
        match value {
            0 => Ok(MaType::Sma),
            1 => Ok(MaType::Ema),
            2 => Ok(MaType::Wma),
            3 => Ok(MaType::Dema),
            4 => Ok(MaType::Tema),
            5 => Ok(MaType::Trima),
            6 => Ok(MaType::Kama),
            7 => Ok(MaType::Mama),
            8 => Ok(MaType::T3),
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
            Self::Sma | Self::Ema | Self::Wma | Self::Trima => period.saturating_sub(1),
            Self::Dema => 2 * period.saturating_sub(1),
            Self::Tema => 3 * period.saturating_sub(1),
            Self::Kama => {
                if period == 1 {
                    0
                } else {
                    period
                }
            }
            Self::Mama => 32,
            Self::T3 => 6 * period.saturating_sub(1),
        }
    }
}

/// 根据 MaType 调度到对应的移动平均计算函数
pub fn compute_ma(input: &[f64], period: usize, ma_type: MaType) -> TaResult<Vec<f64>> {
    if period == 1 {
        return Ok(input.to_vec());
    }
    use crate::overlap;
    match ma_type {
        MaType::Sma => overlap::sma(input, period),
        MaType::Ema => overlap::ema(input, period),
        MaType::Wma => overlap::wma(input, period),
        MaType::Dema => overlap::dema(input, period),
        MaType::Tema => overlap::tema(input, period),
        MaType::Trima => overlap::trima(input, period),
        MaType::Kama => overlap::kama(input, period),
        // MAMA/T3 通过 MA 调度器调用时使用固定默认值，与 C TA-Lib ta_MA.c 完全一致:
        //   MAMA: fastlimit=0.5, slowlimit=0.05 (忽略 period)
        //   T3:   vfactor=0.7 (period 正常传递)
        MaType::Mama => {
            let (mama, _fama) = overlap::mama(input, 0.5, 0.05)?;
            Ok(mama)
        }
        MaType::T3 => overlap::t3(input, period, 0.7),
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
