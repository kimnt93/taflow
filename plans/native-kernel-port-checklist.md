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

## Pending ports

- [ ] `VariableIndexDynamicAverage` (`vidya`)
- [ ] `LaguerreRelativeStrengthIndex` (`laguerre_rsi`)
- [x] `RelativeMomentumIndex` (`rmi`) — Rust streaming kernel and native
  Python adapter verified with warm-up and monotonic-input checks.
- [ ] `JurikMovingAverage` (`jma`)
- [ ] `SSLChannel` (`ssl_channel`)
- [ ] `ParabolicMovingAverageStop` (`pmax`)
- [ ] `TDSequential` (`td_sequential`)
- [ ] `HeikinAshi` (`heikin_ashi`)
- [ ] `AnchoredVolumeWeightedAveragePrice` (`anchored_vwap`)
- [ ] `PivotPoints` (`pivot_points`)
- [ ] `OpeningRange` (`opening_range`)
- [ ] `PremiumDiscount` (`premium_discount`)
- [ ] `EvenBetterSinewave` (`ebsw`)
- [ ] `FibonacciRetracement` (`fibonacci_retracement`)
- [ ] `SessionVolumeLevels` (`session_volume_levels`)
- [ ] `KlingerVolumeOscillator` (`klinger_volume_oscillator`)

These are intentionally not duplicated as Python fallbacks: until ported,
the Python implementations remain visible for compatibility but are not
considered compliant with the native-kernel gate.
