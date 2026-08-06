# STOCHRSI

## Summary

`STOCHRSI` computes Wilder RSI, applies a rolling stochastic range to the RSI
series, and smooths fast %K into fast %D with any TA-Lib moving-average type.
The streaming implementation pipelines persistent RSI and STOCHF states and
does not retain or recompute the complete price history.

Python exposes `taflow.talib.STOCHRSI` for compatible batch calls and
`taflow.StochasticRelativeStrengthIndex` for persistent state.

## Correctness

- Batch and state were checked over 500 values for all nine MA types.
- Both Python namespaces match TA-Lib Python 0.7.1 within `rtol=1e-8` and
  `atol=1e-10`, including KAMA and MAMA.
- The work corrected reusable numerical semantics: RSI now follows TA-Lib's
  exact ratio operation order and flat-input zero rule; stochastic %K uses
  TA-Lib's scaled divisor and epsilon rule; KAMA uses the reference rolling
  update order, epsilon guard, fused recurrence, and period-one identity.
- Chunked continuation, reset/replay, invalid periods, and invalid MA types
  pass.
- Batch, stream, and descriptive implementations are isolated in
  `momentum/stochrsi.rs`, `stream/stochrsi.rs`, and
  `stochastic_relative_strength_index.py`.

## Benchmark status

Performance measurement is intentionally deferred under the implementation-
first project phase. `STOCHRSI.json` records each required benchmark size and
mode as unavailable with that reason; numerical results will be populated in
the later benchmark pass rather than fabricated here.
