use crate::error::{TaError, TaResult};

/// Compute TR, +DM, -DM in a single pass (avoids redundant iteration).
/// Returns (tr, plus_dm, minus_dm) arrays of length `len`, with index 0 = 0.
#[inline]
fn tr_pdm_mdm(high: &[f64], low: &[f64], close: &[f64]) -> (Vec<f64>, Vec<f64>, Vec<f64>) {
    let len = high.len();
    let mut tr = vec![0.0; len];
    let mut pdm = vec![0.0; len];
    let mut mdm = vec![0.0; len];
    if len > 0 {
        tr[0] = high[0] - low[0];
    }
    for i in 1..len {
        // True Range
        let hl = high[i] - low[i];
        let hc = (high[i] - close[i - 1]).abs();
        let lc = (low[i] - close[i - 1]).abs();
        tr[i] = hl.max(hc).max(lc);

        // Directional Movement
        let up = high[i] - high[i - 1];
        let down = low[i - 1] - low[i];
        if up > down && up > 0.0 {
            pdm[i] = up;
        } else if down > up && down > 0.0 {
            mdm[i] = down;
        }
    }
    (tr, pdm, mdm)
}

/// Directional Movement Index (DX)
///
/// DX = 100 * |+DI - -DI| / (+DI + -DI)
///
/// Computes TR, +DM, -DM in a single pass, then applies Wilder smoothing once.
pub fn dx(high: &[f64], low: &[f64], close: &[f64], timeperiod: usize) -> TaResult<Vec<f64>> {
    let len = high.len();
    if len != low.len() || len != close.len() {
        return Err(TaError::LengthMismatch {
            expected: len,
            got: low.len().min(close.len()),
        });
    }
    if timeperiod < 1 || len <= timeperiod {
        return Err(TaError::InsufficientData {
            need: timeperiod + 1,
            got: len,
        });
    }

    let (tr, pdm, mdm) = tr_pdm_mdm(high, low, close);

    // Wilder smoothing seeds
    let mut sum_tr: f64 = tr[1..timeperiod].iter().sum();
    let mut sum_pdm: f64 = pdm[1..timeperiod].iter().sum();
    let mut sum_mdm: f64 = mdm[1..timeperiod].iter().sum();

    let pf = timeperiod as f64;
    let mut output = vec![0.0_f64; len];
    output[..timeperiod].fill(f64::NAN);

    for i in timeperiod..len {
        sum_tr = sum_tr - sum_tr / pf + tr[i];
        sum_pdm = sum_pdm - sum_pdm / pf + pdm[i];
        sum_mdm = sum_mdm - sum_mdm / pf + mdm[i];

        if sum_tr > 0.0 {
            let pdi = 100.0 * sum_pdm / sum_tr;
            let mdi = 100.0 * sum_mdm / sum_tr;
            let sum_di = pdi + mdi;
            output[i] = if sum_di > 0.0 {
                100.0 * (pdi - mdi).abs() / sum_di
            } else {
                0.0
            };
        }
    }

    Ok(output)
}
