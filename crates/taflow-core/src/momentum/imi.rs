use crate::error::{TaError, TaResult};

/// Intraday Momentum Index (IMI).
pub fn imi(open: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = open.len();
    if len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: close.len(),
        });
    }
    if timeperiod < 2 {
        return Err(TaError::InvalidParameter {
            name: "timeperiod",
            value: timeperiod.to_string(),
            reason: "must be >= 2",
        });
    }
    if len < timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod,
            got: len,
        });
    }

    let lookback = timeperiod - 1;
    let mut output = vec![f64::NAN; len];
    for today in lookback..len {
        let mut gains = 0.0;
        let mut losses = 0.0;
        for index in (today + 1 - timeperiod)..=today {
            let movement = close[index] - open[index];
            if movement > 0.0 {
                gains += movement;
            } else {
                losses -= movement;
            }
        }
        output[today] = if gains + losses == 0.0 {
            50.0
        } else {
            100.0 * gains / (gains + losses)
        };
    }
    Ok(output)
}
