# Native-kernel port checklist

`CHECK.md` requires Python to be an adapter only. The following extension
interfaces currently contain numerical Python code and therefore remain
unchecked until their state and batch kernels are implemented in
`crates/taflow-core`, bound in `crates/taflow-python`, and exposed through the
same descriptive Python/Rust name.

For each item:

1. Define a causal Rust state with constructor parameters and aligned input
   series accepted by the Python constructor.
2. Add the Rust batch operator using the same recurrence and warm-up policy.
3. Add the PyO3 state/batch binding and a descriptive doc comment covering
   inputs, parameters, and return values.
4. Replace the Python numerical body with a thin native adapter.
5. Run `cargo check --workspace`, `maturin develop --release`, the adapter
   smoke check, and the benchmark before checking the item here.

## Completed ports

- [x] `VariableIndexDynamicAverage` (`vidya`) — Rust streaming kernel,
  native adapter, and causal history smoke check verified.
- [x] `LaguerreRelativeStrengthIndex` (`laguerre_rsi`) — Rust streaming
  kernel, native adapter, and bounded-output smoke check verified.
- [x] `RelativeMomentumIndex` (`rmi`) — Rust streaming kernel and native
  Python adapter verified with warm-up and monotonic-input checks.
- [x] `JurikMovingAverage` (`jma`) — Rust streaming kernel, native adapter,
  constructor-history and finite-output checks verified.
- [x] `SmoothedTrendChannel` (`ssl_channel`, previously `SSLChannel`) —
  acronym-free native Rust/Python interface verified with multi-output
  warm-up behavior.
- [x] `ParabolicMovingAverageStop` (`pmax`) — Rust EMA/rolling-range stop
  kernel, native two-output adapter, and constructor-history runtime check
  verified.
- [x] `TomDeMarkSequential` (`td_sequential`, previously `TDSequential`) —
  acronym-free Rust setup-count kernel, native two-output adapter, and
  constructor-history runtime check verified.
- [x] `HeikinAshi` (`heikin_ashi`) — Rust causal OHLC transform, native
  four-output adapter, and constructor-history runtime check verified.
- [x] `AnchoredVolumeWeightedAveragePrice` (`anchored_vwap`) — Rust running
  weighted-moment kernel, native three-output adapter, and anchor-history
  runtime check verified.
- [x] `PivotPoints` (`pivot_points`) — Rust causal session-pivot kernel,
  native five-output adapter, and anchor-history runtime check verified.
- [x] `OpeningRange` (`opening_range`) — Rust causal session-range kernel,
  native multi-output adapter, and anchor-history check verified.
- [x] `PremiumDiscount` (`premium_discount`) — Rust rolling range/midpoint
  kernel, native two-output adapter, and constructor-history check verified.
- [x] `EvenBetterSinewave` (`ebsw`) — Rust streaming kernel, native adapter,
  and finite-output smoke check verified.
- [x] `FibonacciRetracement` (`fibonacci_retracement`) — Rust rolling-level
  kernel, native seven-output adapter, and constructor-history check verified.
- [x] `SessionVolumeLevels` (`session_volume_levels`) — Rust fixed-bin
  histogram kernel, native three-output adapter, and constructor-history
  runtime check verified.
- [x] `KlingerVolumeOscillator` (`klinger_volume_oscillator`) — Rust signed
  volume/EMA kernel, native two-output adapter, and constructor-history check
  verified.

All entries above are implemented once in Rust, exposed through PyO3, and
wrapped by thin Python adapters. No numerical Python fallback remains for
these interfaces.
