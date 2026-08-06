# Validation report — 2026-08-06

## Environment

| Component | Value |
|---|---|
| Commit baseline | `cfa51e9` plus this iteration's STOCHRSI work |
| OS | Linux 6.18.7, x86_64 |
| Rust | 1.97.1 |
| Python | 3.12.3 |
| NumPy | 2.5.1 |
| Original oracle | TA-Lib Python 0.7.1 (161 functions) |

## Completed functions in this iteration

| Function | API | Method | Series exercised | Oracle result |
|---|---|---|---:|---|
| ACCBANDS | Rust + Python state | three rolling SMA streams over accelerated high/low and close | batch parity / 1,010,000 benchmark | pass |
| SAR | Rust + Python state | two-bar initialization, acceleration and reversal recurrence | batch parity / 1,010,000 benchmark | pass |
| SAREXT | Rust + Python state | asymmetric acceleration, reversal offset, signed output | batch parity / 1,010,000 benchmark | pass |
| IMI | Rust + Python state | two rolling candle-body sums | batch parity / 1,010,000 benchmark | pass |
| MACDFIX | Rust + Python state | fixed 0.15/0.075 EMAs plus signal EMA | batch parity / 1,010,000 benchmark | pass |
| STOCHF | Rust + Python state | rolling extrema plus selectable fast-D MA | 500 per MA type / 1,010,000 benchmark | pass |
| STOCH | Rust + Python state | rolling extrema plus selectable slow-K/slow-D MAs | 500 per MA pair / 1,010,000 benchmark | pass |
| STOCHRSI | Rust + Python state | exact RSI pipeline into selectable STOCHF smoothing | 500 per MA type; benchmark deferred | pass |
| AVGDEV | batch Python/Rust | per-window mean absolute deviation | 3,000 values | pass |
| SMA | Rust + Python state | O(1) rolling sum | 128 state test / 1,010,000 benchmark | pass |
| EMA | Rust + Python state | SMA seed then EMA recurrence | 128 / 1,010,000 | pass |
| WMA | Rust + Python state | O(1) weighted-sum recurrence | 128 / 1,010,000 | pass |
| DEMA | Rust + Python state | two cascaded seeded EMAs | 128 / 1,010,000 | pass |
| TEMA | Rust + Python state | three cascaded seeded EMAs | 128 / 1,010,000 | pass |
| TRIMA | Rust + Python state | two parity-aware SMA windows | 128 / 1,010,000 | pass |
| KAMA | Rust + Python state | rolling efficiency ratio and adaptive recurrence | 128 / 1,010,000 | pass |
| MAMA/FAMA | Rust + Python state | incremental WMA and alternating Hilbert transforms | 128 / 1,010,000 | pass |
| T3 | Rust + Python state | six cascaded seeded EMAs | 128 / 1,010,000 | pass |
| MA | Rust + Python state | selectable dispatcher over all nine MA types | 200 per MA type / 1,010,000 | pass |
| BBANDS | Rust + Python state | selected middle MA plus rolling population deviation | 200 per MA type / 1,010,000 | pass |
| MIDPOINT | Rust + Python state | monotonic rolling max/min queues | 128 / 1,010,000 | pass |
| MIDPRICE | Rust + Python state | separate high-max and low-min queues | 128 / 1,010,000 | pass |
| RSI | Rust + Python state | Wilder gain/loss recurrence | 128 / 1,010,000 | pass |
| MOM | Rust + Python state | fixed lag ring | 128 / 1,010,000 | pass |
| ROC/ROCP/ROCR/ROCR100 | Rust + Python state | shared fixed-lag recurrence | 128 / 1,010,000 each | pass |
| ATR | Rust + Python state | true range then Wilder recurrence | 128 / 1,010,000 | pass |
| NATR | Rust + Python state | ATR normalized by the current close | 128 / 1,010,000 | pass |
| TRANGE | Rust + Python state | previous-close true range | 128 / 1,010,000 | pass |
| MACD | Rust + Python state | aligned TA-Lib EMA seeds and signal EMA | 128 / 1,010,000 | pass |
| APO/PPO | Rust + Python state | paired MA dispatcher covering all nine MA types | 200 per MA type / 1,010,000 | pass |

`append` returns `None` during each state machine's TA-Lib lookback and
`extend` returns an aligned NumPy output containing `NaN` for that warm-up.

## Commands and results

| Command | Result |
|---|---|
| `cargo test --workspace` | 89 passed |
| `python -m pytest tests/test_stateful.py -q` | 215 passed |
| `python -m pytest tests/test_exhaustive.py -q -k 'ACCBANDS or AVGDEV or IMI'` | 3 passed, 246 deselected |
| `cargo bench -p taflow --bench stream_bench -- --quick` | completed; measurements below |
| `python -m pytest tests/test_exhaustive.py tests/test_stateful.py tests/test_taflow_interface.py -q` | 473 passed |
| `python -m pytest tests/test_full_coverage.py -q` | 620 passed, 310 optional benchmarks skipped |
| `python -m pytest tests/accuracy -q` | 20,270 passed, 1 skipped |
| `python -m pytest tests/accuracy -q -k 'RSI or KAMA or STOCH'` | 752 passed, 19,519 deselected after shared numerical updates |
| `python benches/python_benches/benchmark_function_reports.py --repeats 5` | nine functions × five sizes × four available modes |

The exhaustive suite is green after correcting BBANDS' variance-centre rule and
MACDEXT's aligned-MA seed rule. Its relative tolerance is documented in the
test: the Rust O(n) rolling implementations may use a different accumulation
order from TA-Lib's window rescans while remaining within `rtol=1e-8`.

The price-transform, math-transform, and pointwise-arithmetic state results and
per-function benchmarks are recorded separately in
`reports/2026-08-06-stateless-state.md`.

The rolling extrema, extrema-index, and sum implementation—including duplicate
tie semantics—is detailed in `reports/2026-08-06-rolling-math.md`.

AVGDEV, VAR, and STDDEV numerical methods and measurements are detailed in
`reports/2026-08-06-rolling-statistics.md`.

The complete volume family is detailed in `reports/2026-08-06-volume-state.md`.

BOP and the rolling WILLR/AROON family are detailed in
`reports/2026-08-06-rolling-momentum.md`.

MAMA, T3, and the all-MA-type APO/PPO verification are detailed in
`reports/2026-08-06-adaptive-oscillators.md`.

The selectable MA and all-MA-type BBANDS state verification are detailed in
`reports/2026-08-06-selectable-ma-bbands.md`.

The public Python package is now `taflow`: `taflow.talib` provides the
uppercase batch-compatible surface, while top-level `taflow` exports the
descriptive state classes. Per-function correctness and benchmark artifacts
for the updated surface are in `reports/MA.*`, `reports/BBANDS.*`,
`reports/ACCBANDS.*`, `reports/SAR.*`, `reports/SAREXT.*`, `reports/IMI.*`,
`reports/MACDFIX.*`, `reports/STOCHF.*`, `reports/STOCH.*`, and
`reports/STOCHRSI.*`.

The benchmarked JSON artifacts through STOCH use schema v2 and retain all five wall/CPU
samples and p50/p95/p99/max summaries for 100, 1K, 10K, 100K, and 1M bars,
throughput, isolated-process peak RSS delta, 100K oracle error, and sampled
Python append latency. The future multi-indicator Pipeline is explicitly
reported unavailable; state `extend` is measured separately and is not
misrepresented as a shared execution plan.

Starting with STOCHRSI, the implementation-first phase records required
benchmark cells as explicitly deferred. This preserves the aggregation schema
while function/state/Python coverage is completed before the next benchmark
pass.

## Streaming benchmark

Criterion `--quick`; each sample initializes from 10,000 bars then processes
1,000,000 appended bars.  The timing includes that initialization, so this is
an end-to-end Rust-core measurement, not a Python-call latency claim.

| Indicator | Total time range | Approx. ns per processed bar |
|---|---:|---:|
| SMA(20) | 3.66–3.79 ms | 3.7 |
| EMA(20) | 2.76–2.80 ms | 2.8 |
| WMA(20) | 5.06–5.09 ms | 5.1 |
| DEMA(20) | 5.99–6.05 ms | 6.0 |
| TEMA(20) | 9.51–9.81 ms | 9.6 |
| TRIMA(20) | 7.92–8.30 ms | 8.0 |
| KAMA(20) | 8.70–8.72 ms | 8.7 |
| MAMA(0.5,0.05) | 45.18–46.39 ms | 46.1 |
| T3(20,0.7) | 21.25–21.27 ms | 21.3 |
| APO(12,26,EMA) | 9.49–9.71 ms | 9.7 |
| PPO(12,26,EMA) | 9.49–9.65 ms | 9.6 |
| MA(20,EMA) | 4.84–5.01 ms | 5.0 |
| BBANDS(20,2,2,SMA) | 17.26–17.65 ms | 17.3 |
| ACCBANDS(20) | 16.93–17.06 ms | 17.0 |
| SAR(0.02,0.2) | 8.74–9.07 ms | 8.8 |
| SAREXT(defaults) | 8.66–8.88 ms | 8.7 |
| IMI(14) | 15.03–15.46 ms | 15.1 |
| MACDFIX(9) | 10.34–10.81 ms | 10.4 |
| STOCHF(5,13,SMA) | 33.07–34.40 ms | 33.3 |
| STOCH(5,13,SMA,11,SMA) | 36.78–37.18 ms | 36.9 |
| MIDPOINT(20) | 12.56–13.23 ms | 12.7 |
| MIDPRICE(20) | 24.83–25.11 ms | 24.9 |
| MOM(10) | 2.62–2.66 ms | 2.7 |
| ROC(10) | 2.84–3.00 ms | 2.9 |
| ROCP(10) | 2.58–2.72 ms | 2.7 |
| ROCR(10) | 2.53–2.59 ms | 2.5 |
| ROCR100(10) | 2.67–2.74 ms | 2.7 |
| RSI(14) | 7.09–7.23 ms | 7.1 |
| ATR(14) | 5.41–5.45 ms | 5.4 |
| NATR(14) | 5.56–5.79 ms | 5.7 |
| TRANGE | 2.34–2.41 ms | 2.4 |
| MACD(12,26,9) | 11.89–12.12 ms | 12.1 |

## Next verification gates

1. Keep the now-green 249-case exhaustive suite as a required regression gate
   for all 161 batch-compatible functions.
2. Implement each remaining stateful function in the order and with the
   parity gates in `plans/full-ta-checklist.md`.
3. Add Python-call and growing-history comparison benchmarks; the current
   numbers only prove Rust-core append throughput.
