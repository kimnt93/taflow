# Full TAFlow function coverage checklist

This is the source-derived master inventory of public, same-shape TAFlow computations.
It intentionally does **not** import, enumerate, or count TA-Lib. The inputs are the
Rust stream sources, registered PyO3 classes, Python adapters/exports, and the four
planning checklists named in the audit request.

`x` means a concrete implementation exists in that layer; `_` means the public layer is
missing. A Rust `x` is based on a registered native kernel used by the Python module, not
on a same-spelled token. Naming mismatches are therefore called out separately below.
Enums, output/value structs, adapter classes, metadata helpers, scalar reductions,
look-ahead operations, and internal-only primitives are excluded.

## Audit summary

- Computation surfaces: **300**
- Rust/native implementations: **299**
- Python interfaces: **300**
- Complete in both layers: **299**
- Layer gaps: **1**

## Complete source inventory

| Class | Rust | Py |
|---|:---:|:---:|
| AbsolutePriceOscillator | x | x |
| AccelerationBands | x | x |
| AccumulationDistribution | x | x |
| AccumulationDistributionOscillator | x | x |
| ActiveZoneList | x | x |
| Amihud | x | x |
| AnchoredVolumeWeightedAveragePrice | x | x |
| ArnaudLegouxMovingAverage | x | x |
| Aroon | x | x |
| AroonOscillator | x | x |
| AverageDailyDollarValue | x | x |
| AverageDirectionalIndex | x | x |
| AverageDirectionalIndexRating | x | x |
| AveragePrice | x | x |
| AverageTrueRange | x | x |
| AwesomeOscillator | x | x |
| BalanceOfPower | x | x |
| BarsSince | x | x |
| BollingerBands | x | x |
| BreakOfStructureChangeOfCharacter | x | x |
| CandleAbandonedBaby | x | x |
| CandleAdvanceBlock | x | x |
| CandleBeltHold | x | x |
| CandleBreakaway | x | x |
| CandleClosingMarubozu | x | x |
| CandleConcealBabySwall | x | x |
| CandleCounterAttack | x | x |
| CandleDarkCloudCover | x | x |
| CandleDoji | x | x |
| CandleDojiStar | x | x |
| CandleDragonflyDoji | x | x |
| CandleEngulfing | x | x |
| CandleEveningDojiStar | x | x |
| CandleEveningStar | x | x |
| CandleGapSideSideWhite | x | x |
| CandleGravestoneDoji | x | x |
| CandleHammer | x | x |
| CandleHangingMan | x | x |
| CandleHarami | x | x |
| CandleHaramiCross | x | x |
| CandleHighWave | x | x |
| CandleHikkake | x | x |
| CandleHikkakeModified | x | x |
| CandleHomingPigeon | x | x |
| CandleIdenticalThreeCrows | x | x |
| CandleInNeck | x | x |
| CandleInvertedHammer | x | x |
| CandleKicking | x | x |
| CandleKickingByLength | x | x |
| CandleLadderBottom | x | x |
| CandleLongLeggedDoji | x | x |
| CandleLongLine | x | x |
| CandleMarubozu | x | x |
| CandleMatchingLow | x | x |
| CandleMatHold | x | x |
| CandleMorningDojiStar | x | x |
| CandleMorningStar | x | x |
| CandleOnNeck | x | x |
| CandlePiercing | x | x |
| CandleRickshawman | x | x |
| CandleRiseFallThreeMethods | x | x |
| CandleSeparatingLines | x | x |
| CandleShootingStar | x | x |
| CandleShortLine | x | x |
| CandleSpinningTop | x | x |
| CandleStalledPattern | x | x |
| CandleStickSandwich | x | x |
| CandleTakuri | x | x |
| CandleTasukiGap | x | x |
| CandleThreeBlackCrows | x | x |
| CandleThreeInside | x | x |
| CandleThreeLineStrike | x | x |
| CandleThreeOutside | x | x |
| CandleThreeStarsInSouth | x | x |
| CandleThreeWhiteSoldiers | x | x |
| CandleThrusting | x | x |
| CandleTriStar | x | x |
| CandleTwoCrows | x | x |
| CandleUniqueThreeRiver | x | x |
| CandleUpDownSideGapThreeMethods | x | x |
| CandleUpsideGapTwoCrows | x | x |
| ChaikinMoneyFlow | x | x |
| ChaikinVolatility | x | x |
| ChandeMomentumOscillator | x | x |
| CloseToCloseSigma | x | x |
| CommodityChannelIndex | x | x |
| Cross | x | x |
| Crossover | x | x |
| Crossunder | x | x |
| CumulativeCount | x | x |
| CumulativeMaximum | x | x |
| CumulativeMinimum | x | x |
| CumulativeProduct | x | x |
| CumulativeSum | x | x |
| CumulativeSumControlChart | x | x |
| DecayLinear | x | x |
| DetrendedPriceOscillator | x | x |
| DirectionalMovementIndex | x | x |
| Donchian | x | x |
| DonchianChannels | x | x |
| DoubleExponentialMovingAverage | x | x |
| Drawdown | x | x |
| EaseOfMovement | x | x |
| EntryExit | x | x |
| EqualHighsLows | x | x |
| EvenBetterSinewave | x | x |
| ExponentiallyWeightedCorrelation | x | x |
| ExponentiallyWeightedCovariance | x | x |
| ExponentiallyWeightedStandardDeviation | x | x |
| ExponentiallyWeightedSum | x | x |
| ExponentiallyWeightedVariance | x | x |
| ExponentialMovingAverage | x | x |
| FairValueGap | x | x |
| Falling | x | x |
| FastStochasticOscillator | x | x |
| FibonacciRetracement | x | x |
| FisherTransform | x | x |
| ForceIndex | x | x |
| FracDiff | x | x |
| FractalDimension | x | x |
| GapDown | x | x |
| GapUp | x | x |
| GarmanKlass | x | x |
| GarmanKlassYangZhang | x | x |
| HedgeRatio | x | x |
| HeikinAshi | x | x |
| HigherHigh | x | x |
| HighestSince | x | x |
| HilbertTransformDominantCyclePeriod | x | x |
| HilbertTransformDominantCyclePhase | x | x |
| HilbertTransformPhasor | x | x |
| HilbertTransformSineWave | x | x |
| HilbertTransformTrendline | x | x |
| HilbertTransformTrendMode | x | x |
| HullMovingAverage | x | x |
| Hurst | x | x |
| Ichimoku | x | x |
| InsideBar | x | x |
| IntradayMomentumIndex | x | x |
| JurikMovingAverage | x | x |
| KalmanHedgeRatio | x | x |
| KaufmanAdaptiveMovingAverage | x | x |
| KeltnerChannels | x | x |
| KlingerVolumeOscillator | x | x |
| KnowSureThing | x | x |
| Lag | x | x |
| LaguerreRelativeStrengthIndex | x | x |
| Liquidity | x | x |
| LogReturn | x | x |
| LowerLow | x | x |
| LowestSince | x | x |
| MassIndex | x | x |
| MathAbs | x | x |
| MathAcos | x | x |
| MathAcosh | x | x |
| MathAdd | x | x |
| MathAsin | x | x |
| MathAsinh | x | x |
| MathAtan | x | x |
| MathAtanh | x | x |
| MathCbrt | x | x |
| MathCeil | x | x |
| MathCos | x | x |
| MathCosh | x | x |
| MathCot | x | x |
| MathDegrees | x | x |
| MathDivide | x | x |
| MathExp | x | x |
| MathFloor | x | x |
| MathLn | x | x |
| MathLog10 | x | x |
| MathLog1p | x | x |
| MathMultiply | x | x |
| MathRadians | x | x |
| MathSin | x | x |
| MathSinh | x | x |
| MathSqrt | x | x |
| MathSubtract | x | x |
| MathTan | x | x |
| MathTanh | x | x |
| McGinleyDynamic | x | x |
| MedianPrice | x | x |
| MesaAdaptiveMovingAverage | x | x |
| MinusDirectionalIndicator | x | x |
| MinusDirectionalMovement | x | x |
| Momentum | x | x |
| MoneyFlowIndex | x | x |
| MovingAverage | x | x |
| MovingAverageConvergenceDivergence | x | x |
| MovingAverageConvergenceDivergenceExtended | x | x |
| MovingAverageConvergenceDivergenceFixed | x | x |
| NegativeVolumeIndex | x | x |
| NormalizedAverageTrueRange | x | x |
| OnBalanceVolume | x | x |
| OpeningRange | x | x |
| OrderBlock | x | x |
| OrnsteinUhlenbeckHalfLife | x | x |
| OutsideBar | x | x |
| ParabolicMovingAverageStop | x | x |
| ParabolicSar | x | x |
| ParabolicSarExtended | x | x |
| Parkinson | x | x |
| PercentagePriceOscillator | x | x |
| PivotPoints | x | x |
| PlusDirectionalIndicator | x | x |
| PlusDirectionalMovement | x | x |
| PositionHold | x | x |
| PositiveVolumeIndex | x | x |
| PremiumDiscount | x | x |
| PreviousHighLow | x | x |
| RateOfChange | x | x |
| RateOfChangePercent | x | x |
| RateOfChangeRatio | x | x |
| RateOfChangeRatioPercent | x | x |
| RelativeMomentumIndex | x | x |
| RelativeStrengthIndex | x | x |
| Retracements | x | x |
| Rising | x | x |
| RogersSatchell | x | x |
| RollingAlpha | x | x |
| RollingApply | _ | x |
| RollingArgmax | x | x |
| RollingArgmin | x | x |
| RollingAutocorr | x | x |
| RollingAverageDeviation | x | x |
| RollingBeta | x | x |
| RollingCalmar | x | x |
| RollingCorrelation | x | x |
| RollingCov | x | x |
| RollingEntropy | x | x |
| RollingInformationRatio | x | x |
| RollingInterquartileRange | x | x |
| RollingKurtosis | x | x |
| RollingLinearRegression | x | x |
| RollingLinearRegressionAngle | x | x |
| RollingLinearRegressionIntercept | x | x |
| RollingLinearRegressionSlope | x | x |
| RollingMax | x | x |
| RollingMedian | x | x |
| RollingMidpoint | x | x |
| RollingMidprice | x | x |
| RollingMin | x | x |
| RollingMinMax | x | x |
| RollingMinMaxIndex | x | x |
| RollingMode | x | x |
| RollingPercentile | x | x |
| RollingQuantile | x | x |
| RollingRank | x | x |
| RollingSharpe | x | x |
| RollingSkew | x | x |
| RollingSortino | x | x |
| RollingStandardDeviation | x | x |
| RollingSum | x | x |
| RollingTimeSeriesForecast | x | x |
| RollingVariance | x | x |
| RollingVolumeWeightedAveragePrice | x | x |
| RollingWinsorize | x | x |
| RollingZScore | x | x |
| RollSpread | x | x |
| SchaffTrendCycle | x | x |
| SessionExtrema | x | x |
| SessionFlags | x | x |
| Sessions | x | x |
| SessionVolumeLevels | x | x |
| SignalDelay | x | x |
| SignedPower | x | x |
| SimpleMovingAverage | x | x |
| SmoothedTrendChannel | x | x |
| SpreadZScore | x | x |
| Squeeze | x | x |
| SqueezePro | x | x |
| StochasticOscillator | x | x |
| StochasticRelativeStrengthIndex | x | x |
| Supertrend | x | x |
| SwingHigh | x | x |
| SwingHighLow | x | x |
| SwingHighsLows | x | x |
| SwingLow | x | x |
| TimeSeriesRank | x | x |
| TomDeMarkSequential | x | x |
| TriangularMovingAverage | x | x |
| TripleExponentialAverage | x | x |
| TripleExponentialMovingAverage | x | x |
| TripleExponentialRateOfChange | x | x |
| TrueRange | x | x |
| TrueStrengthIndex | x | x |
| TypicalPrice | x | x |
| UlcerIndex | x | x |
| UltimateOscillator | x | x |
| ValueWhen | x | x |
| VariableIndexDynamicAverage | x | x |
| VariablePeriodMovingAverage | x | x |
| VolumePriceTrend | x | x |
| VolumeWeightedMovingAverage | x | x |
| Vortex | x | x |
| WeightedClose | x | x |
| WeightedMovingAverage | x | x |
| WilliamsPercentR | x | x |
| YangZhang | x | x |
| ZeroLagExponentialMovingAverage | x | x |

## Missing layer report

| Class | Missing | Source evidence / action |
|---|---|---|
| RollingApply | Rust | Python execution helper exists, but there is no Rust kernel. Its arbitrary Python callback prevents a general native kernel; retain as an explicit Python-only exception or narrow its reducer contract. |

`SessionFlags` is present in both layers under `taflow.executions`, so it is not a gap.

## Completed canonical renames

The Rust core, PyO3 states, and Python classes now use the same canonical `Math*`
spellings. The old Rust/PyO3 names are no longer exported.

| Former Rust | Former PyO3 | Canonical class | Status |
|---|---|---|---|
| Add | MathAdd | MathAdd | complete |
| Sub | MathSubtract | MathSubtract | complete |
| Mult | MathMultiply | MathMultiply | complete |
| Div | MathDivide | MathDivide | complete |

The unary math structs were normalized in the same pass (`Acos` → `MathAcos`, ...,
`Tanh` → `MathTanh`).

Other legacy Rust spellings found by the scan should be normalized or retired where a
canonical type already exists: `Linearreg*`, `Tsf`, `RollingMinmax*`,
`MovingAverageConvergenceDivergence`, and `TripleExponentialAverage`.

## Polars same-shape recommendations

The current [Polars Series computation reference](https://docs.pola.rs/api/python/stable/reference/series/computation.html)
was filtered to numeric operations that return an aligned Series and can be causal and
chunk-invariant. The following useful source gaps now have persistent Rust state, PyO3
bindings, and canonical Python classes:

| Recommended class | Rust | Py | Polars analogue |
|---|:---:|:---:|---|
| MathAbs | x | x | `Series.abs` |
| MathAcosh | x | x | `Series.arccosh` |
| MathAsinh | x | x | `Series.arcsinh` |
| MathAtanh | x | x | `Series.arctanh` |
| MathCbrt | x | x | `Series.cbrt` |
| MathCot | x | x | `Series.cot` |
| MathDegrees | x | x | `Series.degrees` |
| MathLog1p | x | x | `Series.log1p` |
| MathRadians | x | x | `Series.radians` |
| CumulativeCount | x | x | `Series.cum_count` |
| ExponentiallyWeightedSum | x | x | `Series.ewm_sum` |

Already covered under canonical TAFlow names: `ewm_mean` →
`ExponentialMovingAverage`, `rolling_mean` → `SimpleMovingAverage`, `diff` →
`Momentum`, `pct_change` → `RateOfChangePercent`, and Polars rolling min/max/sum/median/
quantile/rank/skew/kurtosis/std/var → their `Rolling*` classes. `*_by`, global rank,
scalar aggregations, index-returning methods, and future-dependent peak markers are
excluded from this same-shape causal API.

## Reproducibility

Regenerate this report after interface changes:

```bash
python3 verify/generate_full_ta_functions_checklist.py
```
