# Full TA function implementation checklist

This is the complete TA-only inventory. Extended operators, execution adapters, and
strategy helpers belong in `operator-library-checklist.md` and are
not completion criteria here. Every row is a persistent, same-shape series
interface; scalar-only Polars reductions are excluded. `x` means implemented;
`_` means the layer is still missing. Rust refers to a registered native state
or indicator, and Python refers to a canonical CamelCase class exported by
`taflow`.

Polars computation names were reviewed from the [Series computation API](https://docs.pola.rs/api/python/stable/reference/series/computation.html):
`*_by` and scalar aggregations are excluded. This inventory includes the
TA-Lib math-operator functions because they produce aligned series (for
example `SUM` is `RollingSum`).

| Class | Rust | Python | TA-Lib |
|---|:---:|:---:|---|
| HilbertTransformDominantCyclePeriod | x | x | HT_DCPERIOD |
| HilbertTransformDominantCyclePhase | x | x | HT_DCPHASE |
| HilbertTransformPhasor | x | x | HT_PHASOR |
| HilbertTransformSineWave | x | x | HT_SINE |
| HilbertTransformTrendMode | x | x | HT_TRENDMODE |
| MathAcos | x | x | ACOS |
| MathAsin | x | x | ASIN |
| MathAtan | x | x | ATAN |
| MathCeil | x | x | CEIL |
| MathCos | x | x | COS |
| MathCosh | x | x | COSH |
| MathExp | x | x | EXP |
| MathFloor | x | x | FLOOR |
| MathLn | x | x | LN |
| MathLog10 | x | x | LOG10 |
| MathSin | x | x | SIN |
| MathSinh | x | x | SINH |
| MathSqrt | x | x | SQRT |
| MathTan | x | x | TAN |
| MathTanh | x | x | TANH |
| AverageDirectionalIndex | x | x | ADX |
| AverageDirectionalIndexRating | x | x | ADXR |
| AbsolutePriceOscillator | x | x | APO |
| Aroon | x | x | AROON |
| AroonOscillator | x | x | AROONOSC |
| BalanceOfPower | x | x | BOP |
| CommodityChannelIndex | x | x | CCI |
| ChandeMomentumOscillator | x | x | CMO |
| DirectionalMovementIndex | x | x | DX |
| IntradayMomentumIndex | x | x | IMI |
| MovingAverageConvergenceDivergenceFixed | x | x | MACD |
| MovingAverageConvergenceDivergenceExtended | x | x | MACDEXT |
| MovingAverageConvergenceDivergenceFixed | x | x | MACDFIX |
| MoneyFlowIndex | x | x | MFI |
| MinusDirectionalIndicator | x | x | MINUS_DI |
| MinusDirectionalMovement | x | x | MINUS_DM |
| Momentum | x | x | MOM |
| PlusDirectionalIndicator | x | x | PLUS_DI |
| PlusDirectionalMovement | x | x | PLUS_DM |
| PercentagePriceOscillator | x | x | PPO |
| RateOfChange | x | x | ROC |
| RateOfChangePercent | x | x | ROCP |
| RateOfChangeRatio | x | x | ROCR |
| RateOfChangeRatioPercent | x | x | ROCR100 |
| RelativeStrengthIndex | x | x | RSI |
| StochasticOscillator | x | x | STOCH |
| FastStochasticOscillator | x | x | STOCHF |
| StochasticRelativeStrengthIndex | x | x | STOCHRSI |
| TripleExponentialRateOfChange | x | x | TRIX |
| UltimateOscillator | x | x | ULTOSC |
| WilliamsPercentR | x | x | WILLR |
| AccelerationBands | x | x | ACCBANDS |
| BollingerBands | x | x | BBANDS |
| DoubleExponentialMovingAverage | x | x | DEMA |
| ExponentialMovingAverage | x | x | EMA |
| HilbertTransformTrendline | x | x | HT_TRENDLINE |
| KaufmanAdaptiveMovingAverage | x | x | KAMA |
| MovingAverage | x | x | MA |
| MesaAdaptiveMovingAverage | x | x | MAMA |
| VariablePeriodMovingAverage | x | x | MAVP |
| RollingMidpoint | x | x | MIDPOINT |
| RollingMidprice | x | x | MIDPRICE |
| ParabolicSar | x | x | SAR |
| ParabolicSarExtended | x | x | SAREXT |
| SimpleMovingAverage | x | x | SMA |
| TripleExponentialMovingAverage | x | x | T3/TEMA |
| TriangularMovingAverage | x | x | TRIMA |
| WeightedMovingAverage | x | x | WMA |
| CandleTwoCrows | x | x | CDL2CROWS |
| CandleThreeBlackCrows | x | x | CDL3BLACKCROWS |
| CandleThreeInside | x | x | CDL3INSIDE |
| CandleThreeLineStrike | x | x | CDL3LINESTRIKE |
| CandleThreeOutside | x | x | CDL3OUTSIDE |
| CandleThreeStarsInSouth | x | x | CDL3STARSINSOUTH |
| CandleThreeWhiteSoldiers | x | x | CDL3WHITESOLDIERS |
| CandleAbandonedBaby | x | x | CDLABANDONEDBABY |
| CandleAdvanceBlock | x | x | CDLADVANCEBLOCK |
| CandleBeltHold | x | x | CDLBELTHOLD |
| CandleBreakaway | x | x | CDLBREAKAWAY |
| CandleClosingMarubozu | x | x | CDLCLOSINGMARUBOZU |
| CandleConcealBabySwall | x | x | CDLCONCEALBABYSWALL |
| CandleCounterAttack | x | x | CDLCOUNTERATTACK |
| CandleDarkCloudCover | x | x | CDLDARKCLOUDCOVER |
| CandleDoji | x | x | CDLDOJI |
| CandleDojiStar | x | x | CDLDOJISTAR |
| CandleDragonflyDoji | x | x | CDLDRAGONFLYDOJI |
| CandleEngulfing | x | x | CDLENGULFING |
| CandleEveningDojiStar | x | x | CDLEVENINGDOJISTAR |
| CandleEveningStar | x | x | CDLEVENINGSTAR |
| CandleGapSideSideWhite | x | x | CDLGAPSIDESIDEWHITE |
| CandleGravestoneDoji | x | x | CDLGRAVESTONEDOJI |
| CandleHammer | x | x | CDLHAMMER |
| CandleHangingMan | x | x | CDLHANGINGMAN |
| CandleHarami | x | x | CDLHARAMI |
| CandleHaramiCross | x | x | CDLHARAMICROSS |
| CandleHighWave | x | x | CDLHIGHWAVE |
| CandleHikkake | x | x | CDLHIKKAKE |
| CandleHikkakeModified | x | x | CDLHIKKAKEMOD |
| CandleHomingPigeon | x | x | CDLHOMINGPIGEON |
| CandleIdenticalThreeCrows | x | x | CDLIDENTICAL3CROWS |
| CandleInNeck | x | x | CDLINNECK |
| CandleInvertedHammer | x | x | CDLINVERTEDHAMMER |
| CandleKicking | x | x | CDLKICKING |
| CandleKickingByLength | x | x | CDLKICKINGBYLENGTH |
| CandleLadderBottom | x | x | CDLLADDERBOTTOM |
| CandleLongLeggedDoji | x | x | CDLLONGLEGGEDDOJI |
| CandleLongLine | x | x | CDLLONGLINE |
| CandleMarubozu | x | x | CDLMARUBOZU |
| CandleMatchingLow | x | x | CDLMATCHINGLOW |
| CandleMatHold | x | x | CDLMATHOLD |
| CandleMorningDojiStar | x | x | CDLMORNINGDOJISTAR |
| CandleMorningStar | x | x | CDLMORNINGSTAR |
| CandleOnNeck | x | x | CDLONNECK |
| CandlePiercing | x | x | CDLPIERCING |
| CandleRickshawman | x | x | CDLRICKSHAWMAN |
| CandleRiseFallThreeMethods | x | x | CDLRISEFALL3METHODS |
| CandleSeparatingLines | x | x | CDLSEPARATINGLINES |
| CandleShootingStar | x | x | CDLSHOOTINGSTAR |
| CandleShortLine | x | x | CDLSHORTLINE |
| CandleSpinningTop | x | x | CDLSPINNINGTOP |
| CandleStalledPattern | x | x | CDLSTALLEDPATTERN |
| CandleStickSandwich | x | x | CDLSTICKSANDWICH |
| CandleTakuri | x | x | CDLTAKURI |
| CandleTasukiGap | x | x | CDLTASUKIGAP |
| CandleThrusting | x | x | CDLTHRUSTING |
| CandleTriStar | x | x | CDLTRISTAR |
| CandleUniqueThreeRiver | x | x | CDLUNIQUE3RIVER |
| CandleUpsideGapTwoCrows | x | x | CDLUPSIDEGAP2CROWS |
| CandleUpDownSideGapThreeMethods | x | x | CDLXSIDEGAP3METHODS |
| RollingAverageDeviation | x | x | AVGDEV |
| AveragePrice | x | x | AVGPRICE |
| MedianPrice | x | x | MEDPRICE |
| TypicalPrice | x | x | TYPPRICE |
| WeightedClose | x | x | WCLPRICE |
| RollingBeta | x | x | BETA |
| RollingCorrelation | x | x | CORREL |
| RollingLinearRegression | x | x | LINEARREG |
| RollingLinearRegressionAngle | x | x | LINEARREG_ANGLE |
| RollingLinearRegressionIntercept | x | x | LINEARREG_INTERCEPT |
| RollingLinearRegressionSlope | x | x | LINEARREG_SLOPE |
| RollingStandardDeviation | x | x | STDDEV |
| RollingTimeSeriesForecast | x | x | TSF |
| RollingVariance | x | x | VAR |
| AverageTrueRange | x | x | ATR |
| NormalizedAverageTrueRange | x | x | NATR |
| TrueRange | x | x | TRANGE |
| AccumulationDistribution | x | x | AD |
| AccumulationDistributionOscillator | x | x | ADOSC |
| OnBalanceVolume | x | x | OBV |
| RollingSum | x | x | SUM |
| RollingMin | x | x | MIN |
| RollingMax | x | x | MAX |
| RollingArgmin | x | x | MININDEX |
| RollingArgmax | x | x | MAXINDEX |
| RollingMinMax | x | x | MINMAX |
| RollingMinMaxIndex | x | x | MINMAXINDEX |
| MathAdd | x | x | ADD |
| MathSubtract | x | x | SUB |
| MathMultiply | x | x | MULT |
| MathDivide | x | x | DIV |

## Naming and completion rules

- TA mathematical transforms use `Math*` names (`MathSin`, `MathCos`,
  `MathSinh`, `MathTanh`, ...); bare `Sin`/`Cos`/`Tan` are not canonical.
- `RollingStandardDeviation` is the canonical Python class for TA-Lib
  `STDDEV`; `RollingSum` is the canonical class for `SUM`. No compatibility
  aliases such as `RollingStd` are exported.
- Do not add Polars `*_by` methods or scalar reductions (`sum`, `mean`,
  `arg_max`, etc.) as TAFlow indicators.

## Polars same-shape audit

The Polars Series computation reference includes elementwise transforms and
rolling/ewm series methods such as `rolling_std`, `rolling_var`,
`rolling_kurtosis`, `rolling_skew`, `rolling_sum`, `rolling_min`,
`rolling_max`, `rolling_median`, `rolling_quantile`, `rolling_rank`,
`ewm_std`, and `ewm_var`. These are represented above when they correspond to
TA-Lib functions; the remaining Polars-only methods are intentionally kept
out of this TA checklist. Methods ending in `_by`, scalar reductions, and
index/metadata operations do not satisfy the same-shape TAFlow contract.
