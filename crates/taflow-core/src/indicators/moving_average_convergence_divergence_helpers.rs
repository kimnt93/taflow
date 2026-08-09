use multiversion::multiversion;

use super::moving_average_convergence_divergence::MovingAverageConvergenceDivergenceValue;

/// Advances the fast, slow, and signal EMA recurrences in one steady-state loop.
#[allow(unexpected_cfgs)]
#[multiversion(targets("x86_64+avx2+fma", "x86_64+avx", "x86_64+sse4.2"))]
pub(super) fn macd_ema_steady_loop(
    inputs: &[f64],
    k: [f64; 3],
    state: &mut [f64; 3],
    macd_out: &mut Vec<f64>,
    signal_out: &mut Vec<f64>,
    histogram_out: &mut Vec<f64>,
) -> Option<MovingAverageConvergenceDivergenceValue> {
    let [fast_k, slow_k, signal_k] = k;
    let [mut fast, mut slow, mut signal] = *state;
    let mut last = None;
    for &input in inputs {
        fast = fast_k.mul_add(input - fast, fast);
        slow = slow_k.mul_add(input - slow, slow);
        let macd = fast - slow;
        signal = signal_k.mul_add(macd - signal, signal);
        let histogram = macd - signal;
        macd_out.push(macd);
        signal_out.push(signal);
        histogram_out.push(histogram);
        last = Some(MovingAverageConvergenceDivergenceValue {
            macd,
            signal,
            histogram,
        });
    }
    *state = [fast, slow, signal];
    last
}
