# TAFlow indicator reference

All **287** implemented functions — **161** with a TA-Lib equivalent and **126** extended operators with no TA-Lib counterpart.

Every entry is a class with the same contract: construct it, feed data with `append` (one bar, O(1)) or `extend` (a whole array), and read the full aligned series with `compute()`. Constructor keywords below are the defaults; `Inputs` lists the series each class consumes in positional order.

Correctness for every row is checked against TA-Lib (or pandas, for the rolling and EWM operators) in [../verify/REPORT.md](../verify/REPORT.md); throughput is in [../verify/benchmark_reports/BENCHMARK.md](../verify/benchmark_reports/BENCHMARK.md).

## Contents

- [Moving averages & overlap](#moving-averages--overlap) — 22
- [Momentum & trend](#momentum--trend) — 41
- [Volatility & bands](#volatility--bands) — 18
- [Volume](#volume) — 14
- [Price transforms](#price-transforms) — 7
- [Rolling & statistical operators](#rolling--statistical-operators) — 41
- [Cycle (Hilbert transform)](#cycle-hilbert-transform) — 6
- [Math transforms](#math-transforms) — 28
- [Candlestick patterns](#candlestick-patterns) — 61
- [Market structure & sessions](#market-structure--sessions) — 19
- [Quant & econometrics](#quant--econometrics) — 9
- [Signal & series operators](#signal--series-operators) — 21

## Moving averages & overlap

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `ArnaudLegouxMovingAverage` | — | `timeperiod=5` | close |
| `DoubleExponentialMovingAverage` | DEMA | `timeperiod=30` | price |
| `ExponentialMovingAverage` | EMA | `timeperiod=30` | price |
| `HullMovingAverage` | — | `timeperiod=5` | close |
| `JurikMovingAverage` | — | — | close |
| `KaufmanAdaptiveMovingAverage` | KAMA | `timeperiod=30` | price |
| `McGinleyDynamic` | — | — | close |
| `MesaAdaptiveMovingAverage` | MAMA | `fastlimit=0.5`, `slowlimit=0.05` | price |
| `MovingAverage` | MA | `period=30`, `moving_average_type=0` | price |
| `MovingAverageConvergenceDivergence` | MACD | `fast_period=12`, `slow_period=26`, `signal_period=9` | price |
| `MovingAverageConvergenceDivergenceExtended` | MACDEXT | `fast_period=12`, `fast_average_type=0`, `slow_period=26`, `slow_average_type=0`, `signal_period=9`, `signal_average_type=0` | price |
| `MovingAverageConvergenceDivergenceFixed` | MACDFIX | `signal_period=9` | price |
| `ParabolicMovingAverageStop` | — | — | high, low, close |
| `SimpleMovingAverage` | SMA | `timeperiod=30` | price |
| `TriangularMovingAverage` | TRIMA | `timeperiod=30` | price |
| `TripleExponentialAverage` | T3 | `timeperiod=5`, `volume_factor=0.7` | price |
| `TripleExponentialMovingAverage` | TEMA | `timeperiod=30` | price |
| `TripleExponentialRateOfChange` | TRIX | `timeperiod=30` | price |
| `VariablePeriodMovingAverage` | MAVP | `min_period=2`, `max_period=30`, `average_type=0` | price, periods |
| `VolumeWeightedMovingAverage` | — | `timeperiod=5` | price, volume |
| `WeightedMovingAverage` | WMA | `timeperiod=30` | price |
| `ZeroLagExponentialMovingAverage` | — | `timeperiod=5` | close |

## Momentum & trend

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `Aroon` | AROON | `timeperiod=14` | high, low |
| `AroonOscillator` | AROONOSC | `timeperiod=14` | high, low |
| `AverageDailyDollarValue` | — | — | close, volume |
| `AverageDirectionalIndex` | ADX | `period=14` | high, low, close |
| `AverageDirectionalIndexRating` | ADXR | `period=14` | high, low, close |
| `AwesomeOscillator` | — | — | high, low |
| `BalanceOfPower` | BOP | — | open, high, low, close |
| `ChandeMomentumOscillator` | CMO | `timeperiod=14` | price |
| `CommodityChannelIndex` | CCI | `timeperiod=14` | high, low, close |
| `DirectionalMovementIndex` | DX | `period=14` | high, low, close |
| `EvenBetterSinewave` | — | — | close |
| `FastStochasticOscillator` | STOCHF | `fast_k_period=5`, `fast_d_period=3`, `fast_d_average_type=0` | high, low, close |
| `FisherTransform` | — | — | high, low |
| `Ichimoku` | — | — | high, low, close |
| `IntradayMomentumIndex` | IMI | `period=14` | open, close |
| `KnowSureThing` | — | — | close |
| `MassIndex` | — | — | high, low |
| `MinusDirectionalIndicator` | MINUS_DI | `timeperiod=14` | high, low, close |
| `MinusDirectionalMovement` | MINUS_DM | `timeperiod=14` | high, low |
| `Momentum` | MOM | `timeperiod=10` | price |
| `ParabolicSar` | SAR | `acceleration=0.02`, `maximum=0.2` | high, low |
| `ParabolicSarExtended` | SAREXT | `start_value=0.0`, `offset_on_reverse=0.0`, `acceleration_init_long=0.02`, `acceleration_long=0.02`, `acceleration_max_long=0.2`, `acceleration_init_short=0.02`, `acceleration_short=0.02`, `acceleration_max_short=0.2` | high, low |
| `PlusDirectionalIndicator` | PLUS_DI | `timeperiod=14` | high, low, close |
| `PlusDirectionalMovement` | PLUS_DM | `timeperiod=14` | high, low |
| `RateOfChange` | ROC | `timeperiod=10` | price |
| `RateOfChangePercent` | ROCP | `timeperiod=10` | price |
| `RateOfChangeRatio` | ROCR | `timeperiod=10` | price |
| `RateOfChangeRatioPercent` | ROCR100 | `timeperiod=10` | price |
| `RelativeMomentumIndex` | — | — | close |
| `RelativeStrengthIndex` | RSI | `timeperiod=14` | price |
| `SchaffTrendCycle` | — | — | close |
| `SmoothedTrendChannel` | — | — | high, low, close |
| `StochasticOscillator` | STOCH | `fast_k_period=5`, `slow_k_period=3`, `slow_k_average_type=0`, `slow_d_period=3`, `slow_d_average_type=0` | high, low, close |
| `StochasticRelativeStrengthIndex` | STOCHRSI | `time_period=14`, `fast_k_period=5`, `fast_d_period=3`, `fast_d_average_type=0` | price |
| `TomDeMarkSequential` | — | — | close |
| `TrueStrengthIndex` | — | — | close |
| `UltimateOscillator` | ULTOSC | `timeperiod1=7`, `timeperiod2=14`, `timeperiod3=28` | high, low, close |
| `VariableIndexDynamicAverage` | — | — | close |
| `Vortex` | — | — | high, low, close |
| `WeightedClose` | WCLPRICE | — | high, low, close |
| `WilliamsPercentR` | WILLR | `timeperiod=14` | high, low, close |

## Volatility & bands

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `AccelerationBands` | ACCBANDS | `period=20` | high, low, close |
| `AverageTrueRange` | ATR | `timeperiod=14` | high, low, close |
| `BollingerBands` | BBANDS | `period=5`, `deviations_up=2.0`, `deviations_down=2.0`, `moving_average_type=0` | price |
| `ChaikinVolatility` | — | — | high, low |
| `CloseToCloseSigma` | — | — | close |
| `Donchian` | — | — | high, low |
| `GarmanKlass` | — | — | _open, high, low, close |
| `GarmanKlassYangZhang` | — | — | _open, high, low, close |
| `KeltnerChannels` | — | — | high, low, close |
| `NormalizedAverageTrueRange` | NATR | `timeperiod=14` | high, low, close |
| `Parkinson` | — | — | high, low |
| `RogersSatchell` | — | — | _open, high, low, close |
| `Squeeze` | — | — | high, low, close |
| `SqueezePro` | — | — | high, low, close |
| `Supertrend` | — | — | high, low, close |
| `TrueRange` | TRANGE | — | high, low, close |
| `UlcerIndex` | — | — | close |
| `YangZhang` | — | — | _open, high, low, close |

## Volume

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `AccumulationDistribution` | AD | — | high, low, close, volume |
| `AccumulationDistributionOscillator` | ADOSC | `fastperiod=3`, `slowperiod=10` | high, low, close, volume |
| `Amihud` | — | — | close, volume |
| `AnchoredVolumeWeightedAveragePrice` | — | — | high, low, close, volume, anchor |
| `ChaikinMoneyFlow` | — | — | high, low, close, volume |
| `EaseOfMovement` | — | — | high, low, volume |
| `ForceIndex` | — | — | close, volume |
| `KlingerVolumeOscillator` | — | — | high, low, close, volume |
| `MoneyFlowIndex` | MFI | `timeperiod=14` | high, low, close, volume |
| `NegativeVolumeIndex` | — | — | close, volume |
| `OnBalanceVolume` | OBV | — | price, volume |
| `PositiveVolumeIndex` | — | — | close, volume |
| `SessionVolumeLevels` | — | — | high, low, close, volume, anchor |
| `VolumePriceTrend` | — | — | close, volume |

## Price transforms

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `AbsolutePriceOscillator` | APO | `fastperiod=12`, `slowperiod=26`, `moving_average_type=0` | price |
| `AveragePrice` | AVGPRICE | — | open, high, low, close |
| `DetrendedPriceOscillator` | — | — | close |
| `HeikinAshi` | — | — | _open, high, low, close |
| `MedianPrice` | MEDPRICE | — | high, low |
| `PercentagePriceOscillator` | PPO | `fastperiod=12`, `slowperiod=26`, `moving_average_type=0` | price |
| `TypicalPrice` | TYPPRICE | — | high, low, close |

## Rolling & statistical operators

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `ExponentiallyWeightedCorrelation` | — | `timeperiod=5` | left, right |
| `ExponentiallyWeightedCovariance` | — | `timeperiod=5` | left, right |
| `ExponentiallyWeightedStandardDeviation` | — | `timeperiod=5` | close |
| `ExponentiallyWeightedSum` | — | `timeperiod=5` | close |
| `ExponentiallyWeightedVariance` | — | `timeperiod=5` | close |
| `RollingAlpha` | — | — | close, benchmark |
| `RollingArgmax` | MAXINDEX | `timeperiod=30` | price |
| `RollingArgmin` | MININDEX | `timeperiod=30` | price |
| `RollingAutocorr` | — | — | close |
| `RollingAverageDeviation` | AVGDEV | `timeperiod=14` | price |
| `RollingBeta` | BETA | `timeperiod=5` | price0, price1 |
| `RollingCalmar` | — | `timeperiod=5` | close |
| `RollingCorrelation` | CORREL | `timeperiod=30` | price0, price1 |
| `RollingCov` | — | `timeperiod=5` | left, right |
| `RollingEntropy` | — | — | close |
| `RollingInformationRatio` | — | — | close, benchmark |
| `RollingKurtosis` | — | `timeperiod=5` | close |
| `RollingLinearRegression` | LINEARREG | `timeperiod=14` | price |
| `RollingLinearRegressionAngle` | LINEARREG_ANGLE | `timeperiod=14` | price |
| `RollingLinearRegressionIntercept` | LINEARREG_INTERCEPT | `timeperiod=14` | price |
| `RollingLinearRegressionSlope` | LINEARREG_SLOPE | `timeperiod=14` | price |
| `RollingMax` | MAX | `timeperiod=30` | price |
| `RollingMedian` | — | `timeperiod=5` | close |
| `RollingMidpoint` | MIDPOINT | `timeperiod=14` | price |
| `RollingMidprice` | MIDPRICE | `timeperiod=14` | high, low |
| `RollingMin` | MIN | `timeperiod=30` | price |
| `RollingMinMax` | MINMAX | `timeperiod=30` | price |
| `RollingMinMaxIndex` | MINMAXINDEX | `timeperiod=30` | price |
| `RollingMode` | — | `timeperiod=5` | close |
| `RollingQuantile` | — | `timeperiod=5`, `quantile=0.5` | close |
| `RollingRank` | — | `timeperiod=5` | close |
| `RollingSharpe` | — | `timeperiod=5` | close |
| `RollingSkew` | — | `timeperiod=5` | close |
| `RollingSortino` | — | `timeperiod=5` | close |
| `RollingStandardDeviation` | STDDEV | `timeperiod=5` | price |
| `RollingSum` | SUM | `timeperiod=30` | price |
| `RollingTimeSeriesForecast` | TSF | `timeperiod=14` | price |
| `RollingVariance` | VAR | `timeperiod=5` | price |
| `RollingVolumeWeightedAveragePrice` | — | — | high, low, close, volume |
| `RollingWinsorize` | — | `timeperiod=5` | close |
| `RollingZScore` | — | `timeperiod=5` | close |

## Cycle (Hilbert transform)

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `HilbertTransformDominantCyclePeriod` | HT_DCPERIOD | — | price |
| `HilbertTransformDominantCyclePhase` | HT_DCPHASE | — | price |
| `HilbertTransformPhasor` | HT_PHASOR | — | price |
| `HilbertTransformSineWave` | HT_SINE | — | price |
| `HilbertTransformTrendMode` | HT_TRENDMODE | — | price |
| `HilbertTransformTrendline` | HT_TRENDLINE | — | price |

## Math transforms

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `MathAbs` | — | — | close |
| `MathAcos` | ACOS | — | price |
| `MathAcosh` | — | — | close |
| `MathAdd` | ADD | — | price0, price1 |
| `MathAsin` | ASIN | — | price |
| `MathAsinh` | — | — | close |
| `MathAtan` | ATAN | — | price |
| `MathAtanh` | — | — | close |
| `MathCbrt` | — | — | close |
| `MathCeil` | CEIL | — | price |
| `MathCos` | COS | — | price |
| `MathCosh` | COSH | — | price |
| `MathCot` | — | — | close |
| `MathDegrees` | — | — | close |
| `MathDivide` | DIV | — | price0, price1 |
| `MathExp` | EXP | — | price |
| `MathFloor` | FLOOR | — | price |
| `MathLn` | LN | — | price |
| `MathLog10` | LOG10 | — | price |
| `MathLog1p` | — | — | close |
| `MathMultiply` | MULT | — | price0, price1 |
| `MathRadians` | — | — | close |
| `MathSin` | SIN | — | price |
| `MathSinh` | SINH | — | price |
| `MathSqrt` | SQRT | — | price |
| `MathSubtract` | SUB | — | price0, price1 |
| `MathTan` | TAN | — | price |
| `MathTanh` | TANH | — | price |

## Candlestick patterns

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `CandleAbandonedBaby` | CDLABANDONEDBABY | — | open, high, low, close |
| `CandleAdvanceBlock` | CDLADVANCEBLOCK | — | open, high, low, close |
| `CandleBeltHold` | CDLBELTHOLD | — | open, high, low, close |
| `CandleBreakaway` | CDLBREAKAWAY | — | open, high, low, close |
| `CandleClosingMarubozu` | CDLCLOSINGMARUBOZU | — | open, high, low, close |
| `CandleConcealBabySwall` | CDLCONCEALBABYSWALL | — | open, high, low, close |
| `CandleCounterAttack` | CDLCOUNTERATTACK | — | open, high, low, close |
| `CandleDarkCloudCover` | CDLDARKCLOUDCOVER | — | open, high, low, close |
| `CandleDoji` | CDLDOJI | — | open, high, low, close |
| `CandleDojiStar` | CDLDOJISTAR | — | open, high, low, close |
| `CandleDragonflyDoji` | CDLDRAGONFLYDOJI | — | open, high, low, close |
| `CandleEngulfing` | CDLENGULFING | — | open, high, low, close |
| `CandleEveningDojiStar` | CDLEVENINGDOJISTAR | — | open, high, low, close |
| `CandleEveningStar` | CDLEVENINGSTAR | — | open, high, low, close |
| `CandleGapSideSideWhite` | CDLGAPSIDESIDEWHITE | — | open, high, low, close |
| `CandleGravestoneDoji` | CDLGRAVESTONEDOJI | — | open, high, low, close |
| `CandleHammer` | CDLHAMMER | — | open, high, low, close |
| `CandleHangingMan` | CDLHANGINGMAN | — | open, high, low, close |
| `CandleHarami` | CDLHARAMI | — | open, high, low, close |
| `CandleHaramiCross` | CDLHARAMICROSS | — | open, high, low, close |
| `CandleHighWave` | CDLHIGHWAVE | — | open, high, low, close |
| `CandleHikkake` | CDLHIKKAKE | — | open, high, low, close |
| `CandleHikkakeModified` | CDLHIKKAKEMOD | — | open, high, low, close |
| `CandleHomingPigeon` | CDLHOMINGPIGEON | — | open, high, low, close |
| `CandleIdenticalThreeCrows` | CDLIDENTICAL3CROWS | — | open, high, low, close |
| `CandleInNeck` | CDLINNECK | — | open, high, low, close |
| `CandleInvertedHammer` | CDLINVERTEDHAMMER | — | open, high, low, close |
| `CandleKicking` | CDLKICKING | — | open, high, low, close |
| `CandleKickingByLength` | CDLKICKINGBYLENGTH | — | open, high, low, close |
| `CandleLadderBottom` | CDLLADDERBOTTOM | — | open, high, low, close |
| `CandleLongLeggedDoji` | CDLLONGLEGGEDDOJI | — | open, high, low, close |
| `CandleLongLine` | CDLLONGLINE | — | open, high, low, close |
| `CandleMarubozu` | CDLMARUBOZU | — | open, high, low, close |
| `CandleMatHold` | CDLMATHOLD | — | open, high, low, close |
| `CandleMatchingLow` | CDLMATCHINGLOW | — | open, high, low, close |
| `CandleMorningDojiStar` | CDLMORNINGDOJISTAR | — | open, high, low, close |
| `CandleMorningStar` | CDLMORNINGSTAR | — | open, high, low, close |
| `CandleOnNeck` | CDLONNECK | — | open, high, low, close |
| `CandlePiercing` | CDLPIERCING | — | open, high, low, close |
| `CandleRickshawman` | CDLRICKSHAWMAN | — | open, high, low, close |
| `CandleRiseFallThreeMethods` | CDLRISEFALL3METHODS | — | open, high, low, close |
| `CandleSeparatingLines` | CDLSEPARATINGLINES | — | open, high, low, close |
| `CandleShootingStar` | CDLSHOOTINGSTAR | — | open, high, low, close |
| `CandleShortLine` | CDLSHORTLINE | — | open, high, low, close |
| `CandleSpinningTop` | CDLSPINNINGTOP | — | open, high, low, close |
| `CandleStalledPattern` | CDLSTALLEDPATTERN | — | open, high, low, close |
| `CandleStickSandwich` | CDLSTICKSANDWICH | — | open, high, low, close |
| `CandleTakuri` | CDLTAKURI | — | open, high, low, close |
| `CandleTasukiGap` | CDLTASUKIGAP | — | open, high, low, close |
| `CandleThreeBlackCrows` | CDL3BLACKCROWS | — | open, high, low, close |
| `CandleThreeInside` | CDL3INSIDE | — | open, high, low, close |
| `CandleThreeLineStrike` | CDL3LINESTRIKE | — | open, high, low, close |
| `CandleThreeOutside` | CDL3OUTSIDE | — | open, high, low, close |
| `CandleThreeStarsInSouth` | CDL3STARSINSOUTH | — | open, high, low, close |
| `CandleThreeWhiteSoldiers` | CDL3WHITESOLDIERS | — | open, high, low, close |
| `CandleThrusting` | CDLTHRUSTING | — | open, high, low, close |
| `CandleTriStar` | CDLTRISTAR | — | open, high, low, close |
| `CandleTwoCrows` | CDL2CROWS | — | open, high, low, close |
| `CandleUniqueThreeRiver` | CDLUNIQUE3RIVER | — | open, high, low, close |
| `CandleUpDownSideGapThreeMethods` | CDLXSIDEGAP3METHODS | — | open, high, low, close |
| `CandleUpsideGapTwoCrows` | CDLUPSIDEGAP2CROWS | — | open, high, low, close |

## Market structure & sessions

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `BreakOfStructureChangeOfCharacter` | — | — | high, low, close |
| `EqualHighsLows` | — | — | high, low, close |
| `FairValueGap` | — | — | _open, high, low, close |
| `FibonacciRetracement` | — | — | close |
| `GapDown` | — | — | high, low |
| `GapUp` | — | — | high, low |
| `HigherHigh` | — | — | high, low |
| `InsideBar` | — | — | high, low |
| `Liquidity` | — | — | high, low |
| `LowerLow` | — | — | high, low |
| `OpeningRange` | — | — | high, low, close, anchor |
| `OrderBlock` | — | — | high, low, close, volume |
| `OutsideBar` | — | — | high, low |
| `PivotPoints` | — | — | high, low, close, anchor |
| `PremiumDiscount` | — | — | close |
| `PreviousHighLow` | — | — | new_session, high, low |
| `Retracements` | — | — | high, low, close |
| `Sessions` | — | — | new_session, high, low |
| `SwingHighLow` | — | — | high, low |

## Quant & econometrics

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `CumulativeSumControlChart` | — | — | change |
| `FracDiff` | — | — | close |
| `FractalDimension` | — | — | close |
| `HedgeRatio` | — | — | x, y |
| `Hurst` | — | — | close |
| `KalmanHedgeRatio` | — | — | x, y |
| `OrnsteinUhlenbeckHalfLife` | — | — | price |
| `RollSpread` | — | — | price |
| `SpreadZScore` | — | — | x, y |

## Signal & series operators

| Class | TA-Lib | Parameters | Inputs |
|---|---|---|---|
| `BarsSince` | — | — | condition |
| `Crossover` | — | — | left, right |
| `Crossunder` | — | — | left, right |
| `CumulativeCount` | — | — | close |
| `CumulativeMaximum` | — | — | close |
| `CumulativeMinimum` | — | — | close |
| `CumulativeProduct` | — | — | close |
| `CumulativeSum` | — | — | close |
| `DecayLinear` | — | — | close |
| `Drawdown` | — | — | close |
| `Falling` | — | `timeperiod=5` | close |
| `HighestSince` | — | — | condition, close |
| `Lag` | — | `timeperiod=5` | close |
| `LaguerreRelativeStrengthIndex` | — | — | close |
| `LogReturn` | — | `timeperiod=5` | close |
| `LowestSince` | — | — | condition, close |
| `Rising` | — | `timeperiod=5` | close |
| `SignalDelay` | — | `timeperiod=5` | close |
| `SignedPower` | — | `exponent=5` | close |
| `TimeSeriesRank` | — | `timeperiod=5` | close |
| `ValueWhen` | — | — | condition, close |
