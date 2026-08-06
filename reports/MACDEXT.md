# MACDEXT

## Summary

`MACDEXT` subtracts independently selectable fast and slow moving averages,
then smooths that difference with a third selectable moving average. The
persistent state delays each MA's input start so both first values land on
TA-Lib's shared largest lookback before the signal stream begins.

Python exposes `taflow.talib.MACDEXT` for compatible batch calls and
`taflow.MovingAverageConvergenceDivergenceExtended` for persistent state.

## Correctness

- Batch output was checked against TA-Lib Python 0.7.1 for all 729 combinations
  of the nine fast, slow, and signal MA types over 700 values: pass.
- Rust state versus batch for all 729 combinations: pass.
- Both Python namespaces versus TA-Lib for all 729 combinations: pass.
- Maximum observed absolute error during the direct state audit was
  approximately 3.70e-11, within `rtol=1e-8` and `atol=1e-10`.
- Shared-start alignment correctly covers differing DEMA, TEMA, KAMA, MAMA,
  and T3 lookbacks as well as MACD-compatible all-EMA seeding.
- Chunk continuation, reset/replay, invalid periods, and invalid MA types pass.
- Batch, stream, and descriptive code are isolated in `momentum/macdext.rs`,
  `stream/macdext.rs`, and
  `moving_average_convergence_divergence_extended.py`.

## Benchmark status

Performance measurement is deferred under the implementation-first phase.
`MACDEXT.json` keeps the required aggregation cells explicitly unavailable;
the later benchmark pass will populate measured values.
