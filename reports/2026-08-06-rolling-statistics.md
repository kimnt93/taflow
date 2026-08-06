# Rolling statistics state validation — 2026-08-06

## Implemented functions

| Function | Method | Warm-up | Per-update complexity |
|---|---|---|---:|
| AVGDEV | retain window; recompute mean and absolute deviations newest-to-oldest | period − 1 | O(period) |
| VAR | rolling sum and sum of squares; population variance | period − 1 | O(1) |
| STDDEV | shared rolling variance, non-negative clamp, square root, `nbdev` multiplier | period − 1 | O(1) |
| BETA | percentage-return pairs and rolling cross-moments | period | O(1) |
| CORREL | rolling sums, squares, and cross-product | period − 1 | O(1) |
| LINEARREG family | rolling sum and position-weighted sum | period − 1 | O(1) |

TA-Lib accepts `nbdev` for VAR but does not apply it; the state API preserves
that behavior. All three expose Rust and Python `append`, `extend`, `value`,
and `reset`.

## Correctness evidence

| Test | Series | Result |
|---|---:|---|
| Rust per-bar equality on large-offset oscillating data | 96 values, period 12 | pass |
| Rust constant-series STDDEV | 30 values, period 5 | pass |
| Python extend and append/reset exact equality | 128 values, period 12 | pass |
| Python BETA/CORREL extend and append/reset exact equality | 128 paired values | pass |
| Python five-function regression family exact equality | 128 values, period 14 | pass |
| Complete exhaustive batch plus state suite | 305 tests | pass |
| Rust workspace | 66 tests | pass |

The large-offset input is centered near 1,000,000 to exercise cancellation in
`E[x²] - E[x]²`. The state update uses the same operation order as the batch
implementation and produced exact equality.

## One-million-update benchmark

Criterion `--quick`, release build, period 20, after 10,000 initialization
values. Times include initialization.

| Function | Total time | Approx. ns/update |
|---|---:|---:|
| AVGDEV | 31.35–31.61 ms | 31.4 |
| VAR | 4.97–5.02 ms | 5.0 |
| STDDEV (`nbdev=2`) | 6.07–6.19 ms | 6.1 |
| BETA(20) | 10.93–11.09 ms | 11.0 |
| CORREL(20) | 8.32–8.52 ms | 8.4 |
| LINEARREG(20) | 8.24–8.36 ms | 8.3 |
| LINEARREG_SLOPE(20) | 7.02–7.15 ms | 7.0 |
| LINEARREG_INTERCEPT(20) | 7.54–7.88 ms | 7.6 |
| LINEARREG_ANGLE(20) | 13.08–13.39 ms | 13.1 |
| TSF(20) | 8.51–8.54 ms | 8.5 |
