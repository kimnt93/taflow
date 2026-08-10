# TAFlow versus Wickra: future TA checklist

This is the forward-looking gap list from comparing the current TAFlow public
indicator inventory with Wickra `main` / version `0.9.9`.

The target contract is a causal series indicator: one or more input series go
in, and an aligned output series comes out with one result per input bar
(`NaN` during warm-up). Risk and statistics entries below are therefore
rolling/windowed classes unless the indicator is inherently per-bar. A final
scalar summary over a complete history is not sufficient for this checklist.

The comparison is conceptual rather than a raw name diff. Wickra uses short
names such as `SMA`, `VZO`, and `MaxDrawdown`; the proposed TAFlow names below
are descriptive canonical class names. Existing TAFlow classes with equivalent
semantics are excluded even when their names differ, for example:

- Wickra `AnchoredVWAP` is covered by `AnchoredVolumeWeightedAveragePrice`.
- Wickra `KVO` is covered by `KlingerVolumeOscillator`.
- Wickra `OuHalfLife` is covered by `OrnsteinUhlenbeckHalfLife`.
- TAFlow already has rolling series for Sharpe, Sortino, Calmar, Alpha, Beta,
  Information Ratio, correlation, covariance, variance, skew, kurtosis,
  autocorrelation, and z-score. These are not repeated as scalar Wickra gaps.
- TAFlow's `Drawdown` is already a running-prefix percentage drawdown series.
  `RollingCalmar` computes a rolling maximum drawdown internally, but TAFlow
  does not yet expose that calculation as a standalone rolling series.
- Wickra `SpreadAr1Coefficient` / `PairSpreadZScore` should be compared with
  the existing spread and hedge-ratio series before adding another class.

## Current comparison

| Surface | Current count | Interpretation |
|---|---:|---|
| TAFlow indicator classes | 299 | Current canonical Python class inventory from `verify/function_inventory.json`. |
| TAFlow TA-Lib registry | 161 | TA-Lib mappings, not a count of all TAFlow-only indicators. |
| Wickra indicators | 514 | Includes TA, risk, market breadth, chart-pattern, order-flow, derivatives, alternative-bar, and data-layer features. |
| Direct coverage conclusion | — | Wickra is broader overall, but its count is not a direct replacement target: many Wickra items are outside TAFlow's current scope, while some important risk, volume, statistical, and channel functions are absent from TAFlow. |

Priority means product value for a general technical-analysis library:

- **High** — common in trading, portfolio analytics, or strategy research; add
  after the current TA-Lib/core lifecycle work.
- **Medium** — useful breadth or specialist analysis with a clear audience.
- **Low** — niche, redundant with several existing indicators, or dependent on
  a larger market-data/chart-pattern subsystem.

The backlog currently contains **12 high**, **53 medium**, and **37 low**
priority rows.

For risk/statistics rows, Wickra's name is only the comparison source. If its
implementation is cumulative or summary-oriented, TAFlow should still define
the missing feature as a causal fixed-window series and record the difference
as `VARIANT` during oracle verification.

## Implementation checklist

Implementation progress: the first ten unchecked high-priority entries below
now have canonical Rust states, separate Rust tests, PyO3 bindings, Python
adapters, exports, and lifecycle tests. Their external correctness and
performance gates remain pending by request; the roadmap checkboxes therefore
stay unchecked until those gates are run.

Every row remains unchecked until it has a canonical Rust state, native Python
adapter, one aligned output per input bar, scalar continuation, reset/chunk
invariance, external-oracle comparison where available, and a focused
benchmark.

| Done | Priority | Proposed TAFlow class | Wickra function | Family | Why it belongs on the roadmap |
|:---:|:---:|---|---|---|---|
| [x] | High | `RollingMaximumDrawdown` | `MaxDrawdown` | Risk | MATCH against pandas rolling apply and Wickra 0.9.9; TA-Lib has no equivalent. Focused 1K/10K/100K/1M benchmarks are recorded under `verify/benchmark_reports/`. |
| [ ] | High | `RollingOmegaRatio` | `OmegaRatio` | Risk | Threshold-based return/risk series absent from TAFlow. |
| [ ] | High | `RollingValueAtRisk` | `ValueAtRisk` | Risk | Per-bar rolling loss-quantile series for portfolio and strategy analysis. |
| [ ] | High | `RollingConditionalValueAtRisk` | `ConditionalValueAtRisk` | Risk | Per-bar rolling expected-tail-loss series. |
| [ ] | High | `RollingProfitFactor` | `ProfitFactor` | Performance | Rolling gain/loss quality series over a trade or return window. |
| [ ] | High | `RollingKellyCriterion` | `KellyCriterion` | Position sizing | Rolling position-sizing estimate rather than a final-history scalar. |
| [ ] | High | `RollingRecoveryFactor` | `RecoveryFactor` | Performance | Rolling net-performance-to-drawdown series. |
| [ ] | High | `RollingTreynorRatio` | `TreynorRatio` | Risk | Rolling benchmark-beta-adjusted return series. |
| [ ] | High | `VolumeOscillator` | `VolumeOscillator` | Volume | Widely used volume confirmation indicator. |
| [ ] | High | `VolumeZoneOscillator` | `VZO` | Volume | Common volume-momentum oscillator not currently exposed by TAFlow. |
| [ ] | High | `DemandIndex` | `DemandIndex` | Volume | Established price/volume pressure indicator. |
| [ ] | High | `VolumeRelativeStrengthIndex` | `VolumeRsi` | Volume | Useful volume-specific momentum oscillator. |
| [ ] | Medium | `RollingAverageDrawdown` | `AverageDrawdown` | Risk | Rolling drawdown-severity series. |
| [ ] | Medium | `RollingDrawdownDuration` | `DrawdownDuration` | Risk | Per-bar duration spent below a rolling/prior equity peak. |
| [ ] | Medium | `RollingGainLossRatio` | `GainLossRatio` | Performance | Rolling gain/loss quality series. |
| [ ] | Medium | `RollingPainIndex` | `PainIndex` | Risk | Rolling drawdown-depth series. |
| [ ] | Medium | `RollingVarianceRatio` | `VarianceRatio` | Statistics | Rolling trend-versus-mean-reversion diagnostic. |
| [ ] | Medium | `RollingSpearmanCorrelation` | `SpearmanCorrelation` | Statistics | Rolling rank correlation for nonlinear/outlier-sensitive relationships. |
| [ ] | Medium | `RollingKendallRankCorrelation` | `KendallTau` | Statistics | Rolling robust rank-dependence series. |
| [ ] | Medium | `RollingCointegration` | `Cointegration` | Statistics | Rolling pairs-trading relationship diagnostic. |
| [ ] | Medium | `RollingGrangerCausality` | `GrangerCausality` | Statistics | Rolling lead/lag research series. |
| [ ] | Medium | `RollingLeadLagCrossCorrelation` | `LeadLagCrossCorrelation` | Statistics | Rolling lagged-dependence series. |
| [ ] | Medium | `RollingPairwiseBeta` | `PairwiseBeta` | Statistics | Rolling pairwise beta for relative-value workflows. |
| [ ] | Medium | `RollingBetaNeutralSpread` | `BetaNeutralSpread` | Statistics | Rolling beta-hedged spread series. |
| [ ] | Medium | `RollingMedianAbsoluteDeviation` | `MedianAbsoluteDeviation` | Statistics | Rolling robust dispersion series. |
| [ ] | Medium | `RollingStandardError` | `StandardError` | Statistics | Rolling regression uncertainty series. |
| [ ] | Medium | `RollingCoefficientOfDetermination` | `RSquared` | Statistics | Rolling regression-fit quality series. |
| [ ] | Medium | `VolumeWeightedMovingAverageConvergenceDivergence` | `VolumeWeightedMacd` | Volume | Volume-weighted MACD variant for price/volume confirmation. |
| [ ] | Medium | `BetterVolume` | `BetterVolume` | Volume | Volume classification useful for bar-level strategy filters. |
| [ ] | Medium | `IntradayIntensity` | `IntradayIntensity` | Volume | Measures buying/selling pressure within a bar. |
| [ ] | Medium | `TradeVolumeIndex` | `TradeVolumeIndex` | Volume | Volume trend indicator that tracks directional price changes. |
| [ ] | Medium | `TwiggsMoneyFlow` | `TwiggsMoneyFlow` | Volume | Money-flow alternative with a distinct accumulation/distribution definition. |
| [ ] | Medium | `WilliamsAccumulationDistribution` | `Wad` | Volume | Classic accumulation/distribution indicator. |
| [ ] | Medium | `MarketFacilitationIndex` | `MarketFacilitationIndex` | Volume | Relates price range to volume for activity/regime classification. |
| [ ] | Medium | `TimeSegmentedVolume` | `TSV` | Volume | Volume accumulation split by directional price movement. |
| [ ] | Medium | `MovingAverageEnvelope` | `MaEnvelope` | Bands | Common percentage envelope around a moving average. |
| [ ] | Medium | `AverageTrueRangeBands` | `AtrBands` | Bands | ATR-based dynamic channel useful for stops and breakouts. |
| [ ] | Medium | `LinearRegressionChannel` | `LinRegChannel` | Channels | Common trend/channel construction with regression slope and uncertainty. |
| [ ] | Medium | `StandardErrorBands` | `StandardErrorBands` | Bands | Regression-error bands provide a different contract from Bollinger bands. |
| [ ] | Medium | `DoubleBollingerBands` | `DoubleBollinger` | Bands | Widely used two-volatility-regime Bollinger construction. |
| [ ] | Medium | `ProjectionBands` | `ProjectionBands` | Bands | Forward regression projection channel. |
| [ ] | Medium | `HurstChannel` | `HurstChannel` | Channels | Channel parameters adapt to estimated persistence. |
| [ ] | Medium | `SuperSmoother` | `SuperSmoother` | DSP | Popular low-lag Ehlers smoother. |
| [ ] | Medium | `InverseFisherTransform` | `InverseFisherTransform` | DSP | Common transform used to sharpen oscillator signals. |
| [ ] | Medium | `Decycler` | `Decycler` | DSP | Removes dominant cyclic components from price. |
| [ ] | Medium | `DecyclerOscillator` | `DecyclerOscillator` | DSP | Oscillator form of the decycling filter. |
| [ ] | Medium | `RoofingFilter` | `RoofingFilter` | DSP | Useful band-pass filter for cycle-focused strategies. |
| [ ] | Medium | `CenterOfGravity` | `CenterOfGravity` | DSP | Cycle/turning-point oscillator. |
| [ ] | Medium | `InstantaneousTrendline` | `InstantaneousTrendline` | DSP | Low-lag trend estimator. |
| [ ] | Medium | `AdaptiveCycle` | `AdaptiveCycle` | DSP | Adaptive dominant-cycle estimator. |
| [ ] | Medium | `EhlersStochastic` | `EhlersStochastic` | DSP | Cycle-aware stochastic variant. |
| [ ] | Medium | `HilbertDominantCycle` | `HilbertDominantCycle` | DSP | Consolidated dominant-cycle output beyond TAFlow's individual Hilbert components. |
| [ ] | Medium | `EmpiricalModeDecomposition` | `EmpiricalModeDecomposition` | DSP | Decomposes nonstationary price series into intrinsic modes. |
| [ ] | Medium | `WilliamsFractals` | `WilliamsFractals` | Patterns | Standard fractal swing marker with a well-known definition. |
| [ ] | Medium | `ZigZag` | `ZigZag` | Patterns | Common swing-filtering primitive for chart and structure analysis. |
| [ ] | Medium | `SessionVolumeWeightedAveragePrice` | `SessionVwap` | Sessions | Session-anchored VWAP differs from the existing general/anchored VWAP contract. |
| [ ] | Medium | `SessionHighLow` | `SessionHighLow` | Sessions | Standard session range levels. |
| [ ] | Medium | `SessionRange` | `SessionRange` | Sessions | Session range series for intraday strategies. |
| [ ] | Medium | `OvernightGap` | `OvernightGap` | Sessions | Common open-to-prior-close gap feature. |
| [ ] | Medium | `OvernightIntradayReturn` | `OvernightIntradayReturn` | Sessions | Separates overnight and regular-session returns. |
| [ ] | Medium | `AverageDailyRange` | `AverageDailyRange` | Sessions | Daily range baseline for volatility and position sizing. |
| [ ] | Medium | `TimeOfDayReturnProfile` | `TimeOfDayReturnProfile` | Seasonality | Intraday seasonality profile. |
| [ ] | Medium | `DayOfWeekReturnProfile` | `DayOfWeekProfile` | Seasonality | Day-of-week return/volatility profile. |
| [ ] | Medium | `IntradayVolatilityProfile` | `IntradayVolatilityProfile` | Seasonality | Time-bucketed intraday volatility profile. |
| [ ] | Medium | `VolumeByTimeProfile` | `VolumeByTimeProfile` | Seasonality | Time-of-day volume distribution. |
| [ ] | Low | `QuartileBands` | `QuartileBands` | Bands | Useful robust-band variant, but less common than ATR/regression bands. |
| [ ] | Low | `MedianChannel` | `MedianChannel` | Channels | Robust channel variant for specialist users. |
| [ ] | Low | `AbsoluteBreadthIndex` | `AbsoluteBreadthIndex` | Market breadth | Breadth feature for equity universes; requires multi-symbol input conventions. |
| [ ] | Low | `CumulativeVolumeIndex` | `CumulativeVolumeIndex` | Market breadth | Breadth/volume breadth feature requiring universe-level data. |
| [ ] | Low | `BullishPercentIndex` | `BullishPercentIndex` | Market breadth | Universe breadth indicator, not single-asset TA. |
| [ ] | Low | `UpDownVolumeRatio` | `UpDownVolumeRatio` | Market breadth | Requires aggregated advancing/declining volume. |
| [ ] | Low | `PercentAboveMovingAverage` | `PercentAboveMa` | Market breadth | Requires a symbol universe rather than one OHLCV stream. |
| [ ] | Low | `HighLowIndex` | `HighLowIndex` | Market breadth | Universe-level new-high/new-low breadth. |
| [ ] | Low | `NewHighsNewLows` | `NewHighsNewLows` | Market breadth | Requires cross-sectional market data. |
| [ ] | Low | `BreadthThrust` | `BreadthThrust` | Market breadth | Useful but dependent on a breadth data layer. |
| [ ] | Low | `ArmsIndex` | `Trin` | Market breadth | Classic TRIN/Arms Index; requires breadth inputs. |
| [ ] | Low | `McClellanSummationIndex` | `McClellanSummationIndex` | Market breadth | Requires advance/decline breadth series. |
| [ ] | Low | `McClellanOscillator` | `McClellanOscillator` | Market breadth | Requires advance/decline breadth series. |
| [ ] | Low | `CupAndHandle` | `CupAndHandle` | Chart patterns | Pattern detector with high definition/false-positive risk. |
| [ ] | Low | `RectangleRange` | `RectangleRange` | Chart patterns | Consolidation pattern detector. |
| [ ] | Low | `FlagPennant` | `FlagPennant` | Chart patterns | Pattern detector requiring explicit causal confirmation rules. |
| [ ] | Low | `WedgePattern` | `Wedge` | Chart patterns | Ambiguous pattern definitions; lower priority until pattern framework exists. |
| [ ] | Low | `TrianglePattern` | `Triangle` | Chart patterns | Ambiguous pattern definitions; lower priority until pattern framework exists. |
| [ ] | Low | `HeadAndShoulders` | `HeadAndShoulders` | Chart patterns | Requires swing/pivot confirmation and potentially delayed outputs. |
| [ ] | Low | `TripleTopBottom` | `TripleTopBottom` | Chart patterns | Pattern detector with a broad definition surface. |
| [ ] | Low | `ThreeDrives` | `ThreeDrives` | Harmonic patterns | Specialist harmonic pattern. |
| [ ] | Low | `CypherPattern` | `Cypher` | Harmonic patterns | Specialist harmonic pattern. |
| [ ] | Low | `SharkPattern` | `Shark` | Harmonic patterns | Specialist harmonic pattern. |
| [ ] | Low | `CrabPattern` | `Crab` | Harmonic patterns | Specialist harmonic pattern. |
| [ ] | Low | `BatPattern` | `Bat` | Harmonic patterns | Specialist harmonic pattern. |
| [ ] | Low | `ButterflyPattern` | `Butterfly` | Harmonic patterns | Specialist harmonic pattern. |
| [ ] | Low | `GartleyPattern` | `Gartley` | Harmonic patterns | Specialist harmonic pattern. |
| [ ] | Low | `FourPointHarmonicPattern` | `Abcd` | Harmonic patterns | Specialist four-point harmonic pattern. |
| [ ] | Low | `FibonacciTimeZones` | `FibTimeZones` | Fibonacci | Chart/time projection tool rather than a core causal scalar indicator. |
| [ ] | Low | `FibonacciChannel` | `FibChannel` | Fibonacci | Specialist chart construction. |
| [ ] | Low | `FibonacciArcs` | `FibArcs` | Fibonacci | Specialist chart construction. |
| [ ] | Low | `FibonacciFan` | `FibFan` | Fibonacci | Specialist chart construction. |
| [ ] | Low | `FibonacciConfluence` | `FibConfluence` | Fibonacci | Composite chart-level calculation. |
| [ ] | Low | `GoldenPocket` | `GoldenPocket` | Fibonacci | Specialist retracement zone. |
| [ ] | Low | `AutomaticFibonacci` | `AutoFib` | Fibonacci | Requires a clear swing-selection policy. |
| [ ] | Low | `FibonacciProjection` | `FibProjection` | Fibonacci | Specialist chart construction. |
| [ ] | Low | `FibonacciExtension` | `FibExtension` | Fibonacci | Specialist chart construction. |

## Explicitly deferred from the TA checklist

Wickra also contains useful functionality that should not be added as normal
single-series TA indicators without a separate product decision:

| Area | Wickra examples | Decision |
|---|---|---|
| Order book / microstructure | `Microprice`, `QuotedSpread`, `DepthSlope`, `OrderFlowImbalance`, `Footprint` | Defer until TAFlow defines persistent order-book, trade, and quote input types. |
| Derivatives / crypto data | `FundingRate`, `OpenInterestMomentum`, `LiquidationFeatures`, `TakerBuySellRatio` | Defer until the data model supports derivatives events and exchange timestamps. |
| Alternative bars | `RenkoBars`, `KagiBars`, `PointAndFigureBars`, `RunBars`, `ImbalanceBars` | Defer to a separate bar-builder/data-layer checklist; these are not one-input/one-output indicators. |
| I/O and feeds | `CandleReader`, `Resampler`, `TickAggregator`, `BinanceFeed` | Outside the TAFlow indicator core. |

## Required implementation gates for each checked item

1. Use one descriptive canonical class/state and one snake-case implementation
   file in Rust and Python.
2. Implement `new`, `append`, `value`, `reset`, `extend`, `compute`, and
   `__len__` through the native state lifecycle.
3. Return one aligned output per input bar; for rolling rows, validate the
   window size and define the exact window membership and warm-up position.
4. Define causal warm-up and aligned `NaN` output explicitly.
5. Select an independent oracle: TA-Lib, Polars, pandas, pandas-ta-classic,
   or a pinned licensed implementation.
6. Test cold start, scalar replay, chunked extension, warmed continuation,
   reset/replay, invalid inputs, and multi-output ordering.
7. Add the focused 1K/10K/100K/1M benchmark only after correctness passes.
8. Record `MATCH` or `VARIANT`; do not treat Wickra parity alone as external
   correctness evidence.

## Source references

- [Wickra README and indicator catalogue](https://github.com/wickra-lib/wickra)
- [Wickra architecture and streaming/batch contract](https://github.com/wickra-lib/wickra/blob/main/ARCHITECTURE.md)
- [Wickra core trait implementation](https://github.com/wickra-lib/wickra/blob/main/crates/wickra-core/src/traits.rs)
- [TAFlow generated function inventory](../verify/function_inventory.json)
- [TAFlow implementation checklist](full-ta-checklist.md)
