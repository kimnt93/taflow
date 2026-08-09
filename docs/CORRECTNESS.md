# Correctness and performance

Generated 2026-08-09 from 287 indicator implementations.

Every correctness row uses an external implementation. `VARIANT` means the external calculation was executed and the documented causal or initialization difference was observed; it is not a failed comparison. Speedups are `reference time / TAFlow native-kernel time`, so values above 1× favor TAFlow. A dash means that reference is correctness-only in the timing harness.

## 1. Correctness

| Class | Reference | Correctness | Max error |
|---|---|---:|---:|
| AbsolutePriceOscillator | [TA-Lib: `APO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| AccelerationBands | [TA-Lib: `ACCBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| AccumulationDistribution | [TA-Lib: `AD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| AccumulationDistributionOscillator | [TA-Lib: `ADOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.490e-08` |
| Amihud | [pandas: `pandas.amihud`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `2.895e-24` |
| AnchoredVolumeWeightedAveragePrice | [pandas: `pandas.anchored_vwap`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| ArnaudLegouxMovingAverage | [pandas-ta-classic: `pandas-ta-classic.arnaud_legoux_moving_average`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `5.611e+00` |
| Aroon | [TA-Lib: `AROON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| AroonOscillator | [TA-Lib: `AROONOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.421e-14` |
| AverageDailyDollarValue | [pandas: `pandas.average_daily_dollar_value`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `9.686e-08` |
| AverageDirectionalIndex | [TA-Lib: `ADX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `2.842e-14` |
| AverageDirectionalIndexRating | [TA-Lib: `ADXR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `2.132e-14` |
| AveragePrice | [TA-Lib: `AVGPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `5.684e-14` |
| AverageTrueRange | [TA-Lib: `ATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| AwesomeOscillator | [pandas-ta-classic: `pandas_ta_classic.ao`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `8.527e-14` |
| BalanceOfPower | [TA-Lib: `BOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| BarsSince | [pandas: `pandas.bars_since`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| BollingerBands | [TA-Lib: `BBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `7.927e-10` |
| BreakOfStructureChangeOfCharacter | [smartmoneyconcepts: `smartmoneyconcepts.break_of_structure_change_of_character`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | VARIANT | `0.000e+00` |
| CandleAbandonedBaby | [TA-Lib: `CDLABANDONEDBABY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleAdvanceBlock | [TA-Lib: `CDLADVANCEBLOCK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleBeltHold | [TA-Lib: `CDLBELTHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleBreakaway | [TA-Lib: `CDLBREAKAWAY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleClosingMarubozu | [TA-Lib: `CDLCLOSINGMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleConcealBabySwall | [TA-Lib: `CDLCONCEALBABYSWALL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleCounterAttack | [TA-Lib: `CDLCOUNTERATTACK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleDarkCloudCover | [TA-Lib: `CDLDARKCLOUDCOVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleDoji | [TA-Lib: `CDLDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleDojiStar | [TA-Lib: `CDLDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleDragonflyDoji | [TA-Lib: `CDLDRAGONFLYDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleEngulfing | [TA-Lib: `CDLENGULFING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleEveningDojiStar | [TA-Lib: `CDLEVENINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleEveningStar | [TA-Lib: `CDLEVENINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleGapSideSideWhite | [TA-Lib: `CDLGAPSIDESIDEWHITE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleGravestoneDoji | [TA-Lib: `CDLGRAVESTONEDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleHammer | [TA-Lib: `CDLHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleHangingMan | [TA-Lib: `CDLHANGINGMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleHarami | [TA-Lib: `CDLHARAMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleHaramiCross | [TA-Lib: `CDLHARAMICROSS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleHighWave | [TA-Lib: `CDLHIGHWAVE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleHikkake | [TA-Lib: `CDLHIKKAKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleHikkakeModified | [TA-Lib: `CDLHIKKAKEMOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleHomingPigeon | [TA-Lib: `CDLHOMINGPIGEON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleIdenticalThreeCrows | [TA-Lib: `CDLIDENTICAL3CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleInNeck | [TA-Lib: `CDLINNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleInvertedHammer | [TA-Lib: `CDLINVERTEDHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleKicking | [TA-Lib: `CDLKICKING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleKickingByLength | [TA-Lib: `CDLKICKINGBYLENGTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleLadderBottom | [TA-Lib: `CDLLADDERBOTTOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleLongLeggedDoji | [TA-Lib: `CDLLONGLEGGEDDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleLongLine | [TA-Lib: `CDLLONGLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleMarubozu | [TA-Lib: `CDLMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleMatHold | [TA-Lib: `CDLMATHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleMatchingLow | [TA-Lib: `CDLMATCHINGLOW`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleMorningDojiStar | [TA-Lib: `CDLMORNINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleMorningStar | [TA-Lib: `CDLMORNINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleOnNeck | [TA-Lib: `CDLONNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandlePiercing | [TA-Lib: `CDLPIERCING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleRickshawman | [TA-Lib: `CDLRICKSHAWMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleRiseFallThreeMethods | [TA-Lib: `CDLRISEFALL3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleSeparatingLines | [TA-Lib: `CDLSEPARATINGLINES`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleShootingStar | [TA-Lib: `CDLSHOOTINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleShortLine | [TA-Lib: `CDLSHORTLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleSpinningTop | [TA-Lib: `CDLSPINNINGTOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleStalledPattern | [TA-Lib: `CDLSTALLEDPATTERN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleStickSandwich | [TA-Lib: `CDLSTICKSANDWICH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleTakuri | [TA-Lib: `CDLTAKURI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleTasukiGap | [TA-Lib: `CDLTASUKIGAP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleThreeBlackCrows | [TA-Lib: `CDL3BLACKCROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleThreeInside | [TA-Lib: `CDL3INSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleThreeLineStrike | [TA-Lib: `CDL3LINESTRIKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleThreeOutside | [TA-Lib: `CDL3OUTSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleThreeStarsInSouth | [TA-Lib: `CDL3STARSINSOUTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleThreeWhiteSoldiers | [TA-Lib: `CDL3WHITESOLDIERS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleThrusting | [TA-Lib: `CDLTHRUSTING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleTriStar | [TA-Lib: `CDLTRISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleTwoCrows | [TA-Lib: `CDL2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleUniqueThreeRiver | [TA-Lib: `CDLUNIQUE3RIVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleUpDownSideGapThreeMethods | [TA-Lib: `CDLXSIDEGAP3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| CandleUpsideGapTwoCrows | [TA-Lib: `CDLUPSIDEGAP2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| ChaikinMoneyFlow | [pandas-ta-classic: `pandas_ta_classic.cmf`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `3.993e-14` |
| ChaikinVolatility | [pandas-ta-classic: `pandas-ta-classic.chaikin_volatility`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `2.563e+00` |
| ChandeMomentumOscillator | [TA-Lib: `CMO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `6.817e-14` |
| CloseToCloseSigma | [pandas: `pandas.close_to_close_sigma`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `4.857e-17` |
| CommodityChannelIndex | [TA-Lib: `CCI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.387e-11` |
| Crossover | [pandas-ta-classic: `pandas-ta-classic.crossover`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| Crossunder | [pandas-ta-classic: `pandas-ta-classic.crossunder`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| CumulativeCount | [pandas: `pandas.cumulative_count`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| CumulativeMaximum | [Polars: `Polars.cumulative_maximum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | MATCH | `0.000e+00` |
| CumulativeMinimum | [Polars: `Polars.cumulative_minimum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | MATCH | `0.000e+00` |
| CumulativeProduct | [Polars: `Polars.cumulative_product`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | MATCH | `7.105e-15` |
| CumulativeSum | [Polars: `Polars.cumulative_sum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | MATCH | `0.000e+00` |
| CumulativeSumControlChart | [pandas: `pandas.cumulative_sum_control_chart`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| DecayLinear | [pandas: `pandas.decay_linear`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `2.103e-12` |
| DetrendedPriceOscillator | [pandas-ta-classic: `pandas_ta_classic.dpo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `1.137e-13` |
| DirectionalMovementIndex | [TA-Lib: `DX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `2.842e-14` |
| DonchianChannels | [pandas-ta-classic: `pandas_ta_classic.donchian`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| DoubleExponentialMovingAverage | [TA-Lib: `DEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| Drawdown | [pandas: `pandas.drawdown`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| EaseOfMovement | [pandas-ta-classic: `pandas-ta-classic.ease_of_movement`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `4.531e+03` |
| EqualHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.equal_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | VARIANT | `0.000e+00` |
| EvenBetterSinewave | [pandas-ta-classic: `pandas_ta_classic.ebsw`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| ExponentialMovingAverage | [TA-Lib: `EMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| ExponentiallyWeightedCorrelation | [pandas: `pandas.ewm_corr`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.265e-14` |
| ExponentiallyWeightedCovariance | [pandas: `pandas.ewm_cov`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `2.558e-13` |
| ExponentiallyWeightedStandardDeviation | [pandas: `pandas.ewm_std`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `7.816e-14` |
| ExponentiallyWeightedSum | [pandas: `pandas.exponentially_weighted_sum`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| ExponentiallyWeightedVariance | [Polars: `Polars.ewm_var`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | MATCH | `1.066e-14` |
| FairValueGap | [smartmoneyconcepts: `smartmoneyconcepts.fair_value_gap`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | MATCH | `0.000e+00` |
| Falling | [pandas: `pandas.falling`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| FastStochasticOscillator | [TA-Lib: `STOCHF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| FibonacciRetracement | [pandas: `pandas.fib_retracement`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| FisherTransform | [pandas-ta-classic: `pandas_ta_classic.fisher`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| ForceIndex | [pandas-ta-classic: `pandas_ta_classic.efi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| FracDiff | [pandas: `pandas.frac_diff`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.155e-13` |
| FractalDimension | [pandas: `pandas.fractal_dimension`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.399e-14` |
| GapDown | [pandas: `pandas.gap_down`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| GapUp | [pandas: `pandas.gap_up`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| GarmanKlass | [pandas: `pandas.garman_klass`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.995e-17` |
| GarmanKlassYangZhang | [pandas: `pandas.garman_klass_yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `2.429e-17` |
| HedgeRatio | [pandas: `pandas.hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.353e-09` |
| HeikinAshi | [pandas-ta-classic: `pandas-ta-classic.heikin_ashi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| HigherHigh | [pandas: `pandas.higher_high`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| HighestSince | [pandas: `pandas.highest_since`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| HilbertTransformDominantCyclePeriod | [TA-Lib: `HT_DCPERIOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| HilbertTransformDominantCyclePhase | [TA-Lib: `HT_DCPHASE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| HilbertTransformPhasor | [TA-Lib: `HT_PHASOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| HilbertTransformSineWave | [TA-Lib: `HT_SINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| HilbertTransformTrendMode | [TA-Lib: `HT_TRENDMODE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| HilbertTransformTrendline | [TA-Lib: `HT_TRENDLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| HullMovingAverage | [pandas-ta-classic: `pandas_ta_classic.hma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| Hurst | [pandas: `pandas.hurst`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.388e-14` |
| Ichimoku | [pandas-ta-classic: `pandas-ta-classic.ichimoku`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| InsideBar | [pandas: `pandas.inside_bar`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| IntradayMomentumIndex | [TA-Lib: `IMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.421e-14` |
| JurikMovingAverage | [pandas-ta-classic: `pandas-ta-classic.jma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `4.263e-14` |
| KalmanHedgeRatio | [pandas: `pandas.kalman_hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| KaufmanAdaptiveMovingAverage | [TA-Lib: `KAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `2.842e-14` |
| KeltnerChannels | [pandas-ta-classic: `pandas-ta-classic.keltner_channels`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `3.846e+00` |
| KlingerVolumeOscillator | [pandas-ta-classic: `pandas-ta-classic.kvo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `2.910e-10` |
| KnowSureThing | [pandas-ta-classic: `pandas-ta-classic.know_sure_thing`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `1.824e+04` |
| Lag | [pandas: `pandas.lag`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| LaguerreRelativeStrengthIndex | [pandas: `pandas.laguerre_rsi`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| Liquidity | [smartmoneyconcepts: `smartmoneyconcepts.liquidity`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | VARIANT | `2.000e+00` |
| LogReturn | [pandas-ta-classic: `pandas_ta_classic.log_return`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| LowerLow | [pandas: `pandas.lower_low`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| LowestSince | [pandas: `pandas.lowest_since`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| MassIndex | [pandas-ta-classic: `pandas-ta-classic.mass_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `4.737e-02` |
| MathAbs | [NumPy: `numpy.abs`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `0.000e+00` |
| MathAcos | [TA-Lib: `ACOS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathAcosh | [NumPy: `numpy.arccosh`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `0.000e+00` |
| MathAdd | [TA-Lib: `ADD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathAsin | [TA-Lib: `ASIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathAsinh | [NumPy: `numpy.arcsinh`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `0.000e+00` |
| MathAtan | [TA-Lib: `ATAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathAtanh | [NumPy: `numpy.arctanh`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `1.110e-16` |
| MathCbrt | [NumPy: `numpy.cbrt`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `6.661e-16` |
| MathCeil | [TA-Lib: `CEIL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathCos | [TA-Lib: `COS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathCosh | [TA-Lib: `COSH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathCot | [NumPy: `numpy.tan reciprocal`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `0.000e+00` |
| MathDegrees | [NumPy: `numpy.degrees`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `0.000e+00` |
| MathDivide | [TA-Lib: `DIV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathExp | [TA-Lib: `EXP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathFloor | [TA-Lib: `FLOOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathLn | [TA-Lib: `LN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathLog10 | [TA-Lib: `LOG10`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathLog1p | [NumPy: `numpy.log1p`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `0.000e+00` |
| MathMultiply | [TA-Lib: `MULT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathRadians | [NumPy: `numpy.radians`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `0.000e+00` |
| MathSin | [TA-Lib: `SIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathSinh | [TA-Lib: `SINH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathSqrt | [TA-Lib: `SQRT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathSubtract | [TA-Lib: `SUB`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathTan | [TA-Lib: `TAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MathTanh | [TA-Lib: `TANH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| McGinleyDynamic | [pandas-ta-classic: `pandas-ta-classic.mcginley`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| MedianPrice | [TA-Lib: `MEDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MesaAdaptiveMovingAverage | [TA-Lib: `MAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MinusDirectionalIndicator | [TA-Lib: `MINUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.421e-14` |
| MinusDirectionalMovement | [TA-Lib: `MINUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| Momentum | [TA-Lib: `MOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MoneyFlowIndex | [TA-Lib: `MFI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `2.842e-14` |
| MovingAverage | [TA-Lib: `MA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MovingAverageConvergenceDivergence | [TA-Lib: `MACD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.421e-14` |
| MovingAverageConvergenceDivergenceExtended | [TA-Lib: `MACDEXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| MovingAverageConvergenceDivergenceFixed | [TA-Lib: `MACDFIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `5.684e-14` |
| NegativeVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.negative_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `5.278e+02` |
| NormalizedAverageTrueRange | [TA-Lib: `NATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| OnBalanceVolume | [TA-Lib: `OBV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| OpeningRange | [pandas: `pandas.opening_range`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| OrderBlock | [smartmoneyconcepts: `smartmoneyconcepts.order_block`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | VARIANT | `0.000e+00` |
| OrnsteinUhlenbeckHalfLife | [pandas: `pandas.ornstein_uhlenbeck_half_life`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `3.547e-09` |
| OutsideBar | [pandas: `pandas.outside_bar`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| ParabolicMovingAverageStop | [pandas-ta-classic: `pandas-ta-classic.pmax`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `1.322e+01` |
| ParabolicSar | [TA-Lib: `SAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| ParabolicSarExtended | [TA-Lib: `SAREXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| Parkinson | [pandas: `pandas.parkinson`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `2.602e-17` |
| PercentagePriceOscillator | [TA-Lib: `PPO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| PivotPoints | [pandas: `pandas.pivot_points`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| PlusDirectionalIndicator | [TA-Lib: `PLUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.421e-14` |
| PlusDirectionalMovement | [TA-Lib: `PLUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| PositiveVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.positive_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `4.481e+02` |
| PremiumDiscount | [pandas: `pandas.premium_discount`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| PreviousHighLow | [smartmoneyconcepts: `smartmoneyconcepts.previous_high_low`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | VARIANT | `4.564e+01` |
| RateOfChange | [TA-Lib: `ROC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.421e-14` |
| RateOfChangePercent | [TA-Lib: `ROCP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RateOfChangeRatio | [TA-Lib: `ROCR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RateOfChangeRatioPercent | [TA-Lib: `ROCR100`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RelativeMomentumIndex | [pandas: `pandas.rmi`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| RelativeStrengthIndex | [TA-Lib: `RSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| Retracements | [smartmoneyconcepts: `smartmoneyconcepts.retracements`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | VARIANT | `6.093e+02` |
| Rising | [pandas: `pandas.rising`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| RogersSatchell | [pandas: `pandas.rogers_satchell`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.821e-17` |
| RollSpread | [pandas: `pandas.roll_spread`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.623e-14` |
| RollingAlpha | [pandas: `pandas.rolling_alpha`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `8.285e-10` |
| RollingArgmax | [TA-Lib: `MAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingArgmin | [TA-Lib: `MININDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingAutocorr | [pandas: `pandas.rolling_autocorr`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `6.661e-16` |
| RollingAverageDeviation | [TA-Lib: `AVGDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingBeta | [TA-Lib: `BETA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingCalmar | [pandas: `pandas.rolling_calmar`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.819e-12` |
| RollingCorrelation | [TA-Lib: `CORREL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingCov | [pandas: `pandas.rolling_cov`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.282e-11` |
| RollingEntropy | [pandas: `pandas.rolling_entropy`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `4.441e-16` |
| RollingInformationRatio | [pandas: `pandas.rolling_information_ratio`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `2.886e-11` |
| RollingKurtosis | [pandas: `pandas.rolling_kurtosis`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.332e-15` |
| RollingLinearRegression | [TA-Lib: `LINEARREG`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `5.969e-13` |
| RollingLinearRegressionAngle | [TA-Lib: `LINEARREG_ANGLE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `4.924e-12` |
| RollingLinearRegressionIntercept | [TA-Lib: `LINEARREG_INTERCEPT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `5.969e-13` |
| RollingLinearRegressionSlope | [TA-Lib: `LINEARREG_SLOPE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `9.137e-14` |
| RollingMax | [TA-Lib: `MAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingMedian | [pandas: `pandas.rolling_median`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| RollingMidpoint | [TA-Lib: `MIDPOINT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingMidprice | [TA-Lib: `MIDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingMin | [TA-Lib: `MIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingMinMax | [TA-Lib: `MINMAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingMinMaxIndex | [TA-Lib: `MINMAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingMode | [pandas: `pandas.rolling_mode`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| RollingQuantile | [pandas: `pandas.rolling_quantile`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| RollingRank | [pandas: `pandas.rolling_rank`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| RollingSharpe | [pandas: `pandas.rolling_sharpe`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `6.300e-09` |
| RollingSkew | [pandas: `pandas.rolling_skew`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `8.882e-16` |
| RollingSortino | [pandas: `pandas.rolling_sortino`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| RollingStandardDeviation | [TA-Lib: `STDDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingSum | [TA-Lib: `SUM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingTimeSeriesForecast | [TA-Lib: `TSF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `6.821e-13` |
| RollingVariance | [TA-Lib: `VAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| RollingVolumeWeightedAveragePrice | [pandas: `pandas.rolling_vwap`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `5.684e-14` |
| RollingWinsorize | [pandas: `pandas.rolling_winsorize`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.421e-14` |
| RollingZScore | [pandas: `pandas.rolling_zscore`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `4.771e-14` |
| SchaffTrendCycle | [pandas-ta-classic: `pandas_ta_classic.stc`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `1.000e-08` |
| SessionVolumeLevels | [pandas: `pandas.session_volume_levels`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| Sessions | [smartmoneyconcepts: `smartmoneyconcepts.sessions`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | MATCH | `3.808e-06` |
| SignalDelay | [pandas: `pandas.signal_delay`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| SignedPower | [NumPy: `numpy.sign/numpy.abs/numpy.power`](https://numpy.org/doc/stable/reference/ufuncs.html) | MATCH | `0.000e+00` |
| SimpleMovingAverage | [TA-Lib: `SMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| SmoothedTrendChannel | [pandas: `pandas.ssl_channel`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `1.279e-13` |
| SpreadZScore | [pandas: `pandas.spread_zscore`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `4.680e-14` |
| Squeeze | [pandas-ta-classic: `pandas-ta-classic.squeeze`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `3.553e-15` |
| SqueezePro | [pandas-ta-classic: `pandas-ta-classic.squeeze_pro`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `3.553e-15` |
| StochasticOscillator | [TA-Lib: `STOCH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| StochasticRelativeStrengthIndex | [TA-Lib: `STOCHRSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| Supertrend | [pandas-ta-classic: `pandas-ta-classic.supertrend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `1.421e-14` |
| SwingHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.swing_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | VARIANT | `0.000e+00` |
| TimeSeriesRank | [pandas: `pandas.time_series_rank`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| TomDeMarkSequential | [pandas-ta-classic: `pandas-ta-classic.td_sequential`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| TriangularMovingAverage | [TA-Lib: `TRIMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `6.821e-13` |
| TripleExponentialAverage | [TA-Lib: `T3`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `5.684e-13` |
| TripleExponentialMovingAverage | [TA-Lib: `TEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| TripleExponentialRateOfChange | [TA-Lib: `TRIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.110e-14` |
| TrueRange | [TA-Lib: `TRANGE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| TrueStrengthIndex | [pandas-ta-classic: `pandas-ta-classic.true_strength_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `7.286e+01` |
| TypicalPrice | [TA-Lib: `TYPPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `5.684e-14` |
| UlcerIndex | [pandas-ta-classic: `pandas-ta-classic.ulcer_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `1.169e+01` |
| UltimateOscillator | [TA-Lib: `ULTOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.421e-14` |
| ValueWhen | [pandas: `pandas.value_when`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `0.000e+00` |
| VariableIndexDynamicAverage | [pandas-ta-classic: `pandas-ta-classic.vidya`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `1.421e-14` |
| VariablePeriodMovingAverage | [TA-Lib: `MAVP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `0.000e+00` |
| VolumePriceTrend | [pandas-ta-classic: `pandas-ta-classic.volume_price_trend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | VARIANT | `6.091e+07` |
| VolumeWeightedMovingAverage | [pandas-ta-classic: `pandas_ta_classic.vwma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `5.684e-14` |
| Vortex | [pandas-ta-classic: `pandas-ta-classic.vortex`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `0.000e+00` |
| WeightedClose | [TA-Lib: `WCLPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `5.684e-14` |
| WeightedMovingAverage | [TA-Lib: `WMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `1.857e-10` |
| WilliamsPercentR | [TA-Lib: `WILLR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | MATCH | `2.842e-14` |
| YangZhang | [pandas: `pandas.yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | MATCH | `2.776e-17` |
| ZeroLagExponentialMovingAverage | [pandas-ta-classic: `pandas_ta_classic.zlma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | MATCH | `7.105e-14` |

## 2. Performance on vector

| Class | Reference | 1k bars | 10k bars | 100k bars | 1m bars |
|---|---|---:|---:|---:|---:|
| AccelerationBands | [TA-Lib: `ACCBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.74× | 1.31× | 0.87× | 0.79× |
| MathAcos | [TA-Lib: `ACOS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.39× | 1.28× | 1.03× | 1.00× |
| AccumulationDistribution | [TA-Lib: `AD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.96× | 1.19× | 0.52× | 0.56× |
| MathAdd | [TA-Lib: `ADD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 8.50× | 4.81× | 1.64× | 1.04× |
| AccumulationDistributionOscillator | [TA-Lib: `ADOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.09× | 0.74× | 0.37× | 0.37× |
| AverageDirectionalIndex | [TA-Lib: `ADX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.71× | 1.48× | 1.12× | 1.12× |
| AverageDirectionalIndexRating | [TA-Lib: `ADXR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.99× | 1.17× | 0.94× | 0.92× |
| AbsolutePriceOscillator | [TA-Lib: `APO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.26× | 1.66× | 0.99× | 1.11× |
| Aroon | [TA-Lib: `AROON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.56× | 1.23× | 0.89× | 0.86× |
| AroonOscillator | [TA-Lib: `AROONOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.47× | 1.12× | 0.90× | 0.86× |
| MathAsin | [TA-Lib: `ASIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.45× | 1.42× | 1.10× | 0.88× |
| MathAtan | [TA-Lib: `ATAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.22× | 1.47× | 1.08× | 1.02× |
| AverageTrueRange | [TA-Lib: `ATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.44× | 1.69× | 1.23× | 1.17× |
| RollingAverageDeviation | [TA-Lib: `AVGDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.41× | 1.04× | 0.91× | 0.91× |
| AveragePrice | [TA-Lib: `AVGPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.59× | 1.38× | 0.44× | 0.62× |
| BollingerBands | [TA-Lib: `BBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 9.00× | 2.57× | 1.40× | 1.01× |
| RollingBeta | [TA-Lib: `BETA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.80× | 1.53× | 1.13× | 1.09× |
| BalanceOfPower | [TA-Lib: `BOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.19× | 1.36× | 0.54× | 0.63× |
| CommodityChannelIndex | [TA-Lib: `CCI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.20× | 1.17× | 0.95× | 1.10× |
| CandleTwoCrows | [TA-Lib: `CDL2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.71× | 0.89× | 1.02× | 0.96× |
| CandleThreeBlackCrows | [TA-Lib: `CDL3BLACKCROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.36× | 1.49× | 0.89× | 0.84× |
| CandleThreeInside | [TA-Lib: `CDL3INSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.20× | 1.44× | 1.05× | 1.02× |
| CandleThreeLineStrike | [TA-Lib: `CDL3LINESTRIKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.11× | 1.61× | 0.94× | 1.02× |
| CandleThreeOutside | [TA-Lib: `CDL3OUTSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.99× | 1.30× | 0.83× | 0.78× |
| CandleThreeStarsInSouth | [TA-Lib: `CDL3STARSINSOUTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.36× | 1.88× | 1.31× | 1.24× |
| CandleThreeWhiteSoldiers | [TA-Lib: `CDL3WHITESOLDIERS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.39× | 2.57× | 2.09× | 1.96× |
| CandleAbandonedBaby | [TA-Lib: `CDLABANDONEDBABY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.34× | 1.32× | 1.02× | 0.98× |
| CandleAdvanceBlock | [TA-Lib: `CDLADVANCEBLOCK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.90× | 2.87× | 2.42× | 2.58× |
| CandleBeltHold | [TA-Lib: `CDLBELTHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.57× | 1.47× | 1.07× | 1.06× |
| CandleBreakaway | [TA-Lib: `CDLBREAKAWAY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.98× | 1.21× | 0.74× | 0.79× |
| CandleClosingMarubozu | [TA-Lib: `CDLCLOSINGMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.43× | 1.39× | 1.07× | 1.04× |
| CandleConcealBabySwall | [TA-Lib: `CDLCONCEALBABYSWALL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.86× | 2.21× | 1.22× | 1.22× |
| CandleCounterAttack | [TA-Lib: `CDLCOUNTERATTACK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.20× | 1.94× | 1.18× | 1.06× |
| CandleDarkCloudCover | [TA-Lib: `CDLDARKCLOUDCOVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.08× | 1.99× | 0.98× | 0.91× |
| CandleDoji | [TA-Lib: `CDLDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.92× | 1.74× | 0.82× | 0.83× |
| CandleDojiStar | [TA-Lib: `CDLDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.43× | 1.41× | 0.90× | 0.96× |
| CandleDragonflyDoji | [TA-Lib: `CDLDRAGONFLYDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.68× | 2.01× | 1.27× | 1.29× |
| CandleEngulfing | [TA-Lib: `CDLENGULFING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.10× | 1.51× | 0.86× | 0.83× |
| CandleEveningDojiStar | [TA-Lib: `CDLEVENINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.76× | 1.49× | 0.98× | 1.01× |
| CandleEveningStar | [TA-Lib: `CDLEVENINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.60× | 1.26× | 0.97× | 0.90× |
| CandleGapSideSideWhite | [TA-Lib: `CDLGAPSIDESIDEWHITE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 6.42× | 4.18× | 3.47× | 3.14× |
| CandleGravestoneDoji | [TA-Lib: `CDLGRAVESTONEDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.46× | 1.95× | 1.45× | 1.28× |
| CandleHammer | [TA-Lib: `CDLHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.70× | 1.45× | 1.16× | 1.11× |
| CandleHangingMan | [TA-Lib: `CDLHANGINGMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.88× | 1.43× | 1.15× | 1.14× |
| CandleHarami | [TA-Lib: `CDLHARAMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.40× | 1.91× | 1.19× | 1.19× |
| CandleHaramiCross | [TA-Lib: `CDLHARAMICROSS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.28× | 1.54× | 1.06× | 0.97× |
| CandleHighWave | [TA-Lib: `CDLHIGHWAVE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.35× | 1.43× | 1.14× | 1.16× |
| CandleHikkake | [TA-Lib: `CDLHIKKAKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.47× | 1.37× | 0.84× | 0.76× |
| CandleHikkakeModified | [TA-Lib: `CDLHIKKAKEMOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.39× | 1.35× | 1.00× | 0.94× |
| CandleHomingPigeon | [TA-Lib: `CDLHOMINGPIGEON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.26× | 1.77× | 1.17× | 1.14× |
| CandleIdenticalThreeCrows | [TA-Lib: `CDLIDENTICAL3CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.47× | 1.92× | 1.13× | 1.11× |
| CandleInNeck | [TA-Lib: `CDLINNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.76× | 2.06× | 1.05× | 1.04× |
| CandleInvertedHammer | [TA-Lib: `CDLINVERTEDHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.78× | 1.58× | 1.25× | 1.12× |
| CandleKicking | [TA-Lib: `CDLKICKING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.49× | 1.93× | 1.59× | 1.44× |
| CandleKickingByLength | [TA-Lib: `CDLKICKINGBYLENGTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.99× | 2.24× | 1.41× | 1.36× |
| CandleLadderBottom | [TA-Lib: `CDLLADDERBOTTOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.53× | 1.47× | 1.09× | 0.99× |
| CandleLongLeggedDoji | [TA-Lib: `CDLLONGLEGGEDDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.25× | 1.76× | 1.26× | 1.20× |
| CandleLongLine | [TA-Lib: `CDLLONGLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.10× | 1.62× | 1.30× | 1.28× |
| CandleMarubozu | [TA-Lib: `CDLMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.57× | 1.40× | 1.08× | 1.00× |
| CandleMatchingLow | [TA-Lib: `CDLMATCHINGLOW`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.14× | 1.85× | 1.30× | 1.11× |
| CandleMatHold | [TA-Lib: `CDLMATHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.57× | 1.34× | 0.94× | 0.92× |
| CandleMorningDojiStar | [TA-Lib: `CDLMORNINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.89× | 1.51× | 0.97× | 0.95× |
| CandleMorningStar | [TA-Lib: `CDLMORNINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.73× | 1.38× | 0.96× | 0.91× |
| CandleOnNeck | [TA-Lib: `CDLONNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.89× | 1.87× | 1.05× | 1.03× |
| CandlePiercing | [TA-Lib: `CDLPIERCING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.22× | 1.61× | 1.17× | 1.02× |
| CandleRickshawman | [TA-Lib: `CDLRICKSHAWMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.52× | 1.77× | 1.31× | 1.27× |
| CandleRiseFallThreeMethods | [TA-Lib: `CDLRISEFALL3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.48× | 1.32× | 0.93× | 0.92× |
| CandleSeparatingLines | [TA-Lib: `CDLSEPARATINGLINES`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.40× | 1.88× | 1.34× | 1.23× |
| CandleShootingStar | [TA-Lib: `CDLSHOOTINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.54× | 1.59× | 1.24× | 1.16× |
| CandleShortLine | [TA-Lib: `CDLSHORTLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.83× | 1.70× | 1.34× | 1.32× |
| CandleSpinningTop | [TA-Lib: `CDLSPINNINGTOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.34× | 1.40× | 1.03× | 0.99× |
| CandleStalledPattern | [TA-Lib: `CDLSTALLEDPATTERN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.74× | 2.29× | 1.86× | 1.74× |
| CandleStickSandwich | [TA-Lib: `CDLSTICKSANDWICH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.79× | 2.05× | 1.07× | 1.03× |
| CandleTakuri | [TA-Lib: `CDLTAKURI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.83× | 1.93× | 1.31× | 1.23× |
| CandleTasukiGap | [TA-Lib: `CDLTASUKIGAP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.30× | 2.11× | 1.81× | 1.73× |
| CandleThrusting | [TA-Lib: `CDLTHRUSTING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.45× | 1.80× | 1.14× | 1.04× |
| CandleTriStar | [TA-Lib: `CDLTRISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.19× | 1.96× | 1.07× | 1.02× |
| CandleUniqueThreeRiver | [TA-Lib: `CDLUNIQUE3RIVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.31× | 1.29× | 0.65× | 0.64× |
| CandleUpsideGapTwoCrows | [TA-Lib: `CDLUPSIDEGAP2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.25× | 1.45× | 1.16× | 1.03× |
| CandleUpDownSideGapThreeMethods | [TA-Lib: `CDLXSIDEGAP3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.91× | 1.35× | 1.00× | 0.97× |
| MathCeil | [TA-Lib: `CEIL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 6.86× | 1.81× | 0.73× | 0.68× |
| ChandeMomentumOscillator | [TA-Lib: `CMO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.35× | 1.78× | 1.14× | 1.09× |
| RollingCorrelation | [TA-Lib: `CORREL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.80× | 1.85× | 1.13× | 1.19× |
| MathCos | [TA-Lib: `COS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.31× | 1.22× | 1.17× | 1.02× |
| MathCosh | [TA-Lib: `COSH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.39× | 1.53× | 1.08× | 1.13× |
| DoubleExponentialMovingAverage | [TA-Lib: `DEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.87× | 1.55× | 1.17× | 1.99× |
| MathDivide | [TA-Lib: `DIV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 8.30× | 4.08× | 1.50× | 0.86× |
| DirectionalMovementIndex | [TA-Lib: `DX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.17× | 1.19× | 0.88× | 0.84× |
| ExponentialMovingAverage | [TA-Lib: `EMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 8.95× | 2.19× | 1.24× | 1.01× |
| MathExp | [TA-Lib: `EXP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.62× | 1.45× | 1.02× | 0.96× |
| MathFloor | [TA-Lib: `FLOOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 7.14× | 1.92× | 0.76× | 0.72× |
| HilbertTransformDominantCyclePeriod | [TA-Lib: `HT_DCPERIOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.08× | 1.04× | 0.94× |
| HilbertTransformDominantCyclePhase | [TA-Lib: `HT_DCPHASE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.43× | 4.32× | 4.29× | 3.80× |
| HilbertTransformPhasor | [TA-Lib: `HT_PHASOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.61× | 1.11× | 0.97× | 1.04× |
| HilbertTransformSineWave | [TA-Lib: `HT_SINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.39× | 3.26× | 3.23× | 3.38× |
| HilbertTransformTrendline | [TA-Lib: `HT_TRENDLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.14× | 0.93× | 0.79× | 0.80× |
| HilbertTransformTrendMode | [TA-Lib: `HT_TRENDMODE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.83× | 2.85× | 2.69× | 2.88× |
| IntradayMomentumIndex | [TA-Lib: `IMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.93× | 5.11× | 4.95× | 4.61× |
| KaufmanAdaptiveMovingAverage | [TA-Lib: `KAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 6.55× | 1.58× | 1.13× | 0.98× |
| RollingLinearRegression | [TA-Lib: `LINEARREG`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.62× | 1.21× | 1.00× | 0.95× |
| RollingLinearRegressionAngle | [TA-Lib: `LINEARREG_ANGLE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.48× | 1.17× | 0.94× | 0.96× |
| RollingLinearRegressionIntercept | [TA-Lib: `LINEARREG_INTERCEPT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.11× | 1.23× | 1.11× | 0.96× |
| RollingLinearRegressionSlope | [TA-Lib: `LINEARREG_SLOPE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.13× | 1.22× | 1.00× | 0.97× |
| MathLn | [TA-Lib: `LN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.97× | 1.57× | 1.00× | 1.06× |
| MathLog10 | [TA-Lib: `LOG10`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.49× | 1.32× | 1.02× | 0.88× |
| MovingAverage | [TA-Lib: `MA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 6.19× | 1.22× | 0.52× | 0.46× |
| MovingAverageConvergenceDivergence | [TA-Lib: `MACD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 9.99× | 5.33× | 4.27× | 4.38× |
| MovingAverageConvergenceDivergenceExtended | [TA-Lib: `MACDEXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.45× | 1.11× | 0.79× | 0.84× |
| MovingAverageConvergenceDivergenceFixed | [TA-Lib: `MACDFIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 9.87× | 4.96× | 4.10× | 4.33× |
| MesaAdaptiveMovingAverage | [TA-Lib: `MAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.57× | 1.01× | 0.98× | 0.92× |
| VariablePeriodMovingAverage | [TA-Lib: `MAVP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.93× | 0.67× | 0.69× | 0.78× |
| RollingMax | [TA-Lib: `MAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 7.20× | 2.43× | 1.55× | 1.29× |
| RollingArgmax | [TA-Lib: `MAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.78× | 1.84× | 1.34× | 1.26× |
| MedianPrice | [TA-Lib: `MEDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 6.36× | 1.96× | 0.50× | 0.53× |
| MoneyFlowIndex | [TA-Lib: `MFI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.34× | 1.81× | 1.56× | 1.58× |
| RollingMidpoint | [TA-Lib: `MIDPOINT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.07× | 1.33× | 0.88× | 0.79× |
| RollingMidprice | [TA-Lib: `MIDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.73× | 1.38× | 0.97× | 0.81× |
| RollingMin | [TA-Lib: `MIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 9.75× | 2.42× | 1.66× | 1.35× |
| RollingArgmin | [TA-Lib: `MININDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.84× | 1.94× | 1.35× | 1.30× |
| RollingMinMax | [TA-Lib: `MINMAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.72× | 1.76× | 1.24× | 1.02× |
| RollingMinMaxIndex | [TA-Lib: `MINMAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.20× | 1.56× | 1.26× | 1.15× |
| MinusDirectionalIndicator | [TA-Lib: `MINUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.10× | 0.95× | 0.71× | 0.81× |
| MinusDirectionalMovement | [TA-Lib: `MINUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.20× | 1.73× | 1.03× | 0.97× |
| Momentum | [TA-Lib: `MOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 8.02× | 1.66× | 0.37× | 0.33× |
| MathMultiply | [TA-Lib: `MULT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 9.40× | 4.77× | 1.65× | 1.06× |
| NormalizedAverageTrueRange | [TA-Lib: `NATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.86× | 1.39× | 0.93× | 0.96× |
| OnBalanceVolume | [TA-Lib: `OBV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.93× | 1.61× | 1.00× | 0.87× |
| PlusDirectionalIndicator | [TA-Lib: `PLUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.05× | 1.59× | 1.22× | 1.15× |
| PlusDirectionalMovement | [TA-Lib: `PLUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.76× | 1.48× | 1.06× | 0.98× |
| PercentagePriceOscillator | [TA-Lib: `PPO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.55× | 1.96× | 1.32× | 1.21× |
| RateOfChange | [TA-Lib: `ROC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 7.46× | 1.83× | 0.67× | 0.58× |
| RateOfChangePercent | [TA-Lib: `ROCP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 7.91× | 1.78× | 0.69× | 0.53× |
| RateOfChangeRatio | [TA-Lib: `ROCR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 7.49× | 1.94× | 0.80× | 0.55× |
| RateOfChangeRatioPercent | [TA-Lib: `ROCR100`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 7.65× | 2.01× | 0.69× | 0.64× |
| RelativeStrengthIndex | [TA-Lib: `RSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.70× | 1.46× | 0.98× | 0.95× |
| ParabolicSar | [TA-Lib: `SAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.65× | 0.77× | 0.59× | 0.55× |
| ParabolicSarExtended | [TA-Lib: `SAREXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.05× | 0.92× | 0.58× | 0.55× |
| MathSin | [TA-Lib: `SIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.22× | 1.24× | 1.06× | 1.01× |
| MathSinh | [TA-Lib: `SINH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 4.10× | 1.39× | 1.09× | 1.04× |
| SimpleMovingAverage | [TA-Lib: `SMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 7.63× | 2.33× | 1.16× | 0.91× |
| MathSqrt | [TA-Lib: `SQRT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 9.69× | 4.34× | 2.28× | 1.88× |
| RollingStandardDeviation | [TA-Lib: `STDDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.79× | 1.52× | 0.77× | 0.79× |
| StochasticOscillator | [TA-Lib: `STOCH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.61× | 1.21× | 0.89× | 0.81× |
| FastStochasticOscillator | [TA-Lib: `STOCHF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.82× | 0.86× | 0.63× | 0.84× |
| StochasticRelativeStrengthIndex | [TA-Lib: `STOCHRSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.42× | 0.88× | 0.67× | 0.65× |
| MathSubtract | [TA-Lib: `SUB`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 9.32× | 6.09× | 1.71× | 1.09× |
| RollingSum | [TA-Lib: `SUM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.87× | 1.41× | 0.77× | 0.63× |
| TripleExponentialAverage | [TA-Lib: `T3`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 8.00× | 2.15× | 1.20× | 1.18× |
| MathTan | [TA-Lib: `TAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.60× | 1.13× | 1.01× | 1.01× |
| MathTanh | [TA-Lib: `TANH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.93× | 1.90× | 1.10× | 0.99× |
| TripleExponentialMovingAverage | [TA-Lib: `TEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.63× | 1.25× | 0.96× | 1.12× |
| TrueRange | [TA-Lib: `TRANGE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 6.01× | 2.16× | 0.78× | 0.94× |
| TriangularMovingAverage | [TA-Lib: `TRIMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.24× | 1.39× | 0.81× | 0.72× |
| TripleExponentialRateOfChange | [TA-Lib: `TRIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 7.34× | 4.98× | 4.06× | 4.08× |
| RollingTimeSeriesForecast | [TA-Lib: `TSF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.75× | 1.17× | 0.90× | 1.03× |
| TypicalPrice | [TA-Lib: `TYPPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.78× | 1.99× | 0.58× | 0.64× |
| UltimateOscillator | [TA-Lib: `ULTOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.05× | 1.45× | 1.21× | 1.10× |
| RollingVariance | [TA-Lib: `VAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.74× | 1.59× | 0.69× | 0.64× |
| WeightedClose | [TA-Lib: `WCLPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.63× | 1.98× | 0.65× | 0.62× |
| WilliamsPercentR | [TA-Lib: `WILLR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.73× | 1.26× | 0.99× | 0.86× |
| WeightedMovingAverage | [TA-Lib: `WMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.24× | 1.43× | 0.68× | 0.61× |
| Amihud | [pandas: `pandas.amihud`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| AnchoredVolumeWeightedAveragePrice | [pandas: `pandas.anchored_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| ArnaudLegouxMovingAverage | [pandas-ta-classic: `pandas-ta-classic.arnaud_legoux_moving_average`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| AverageDailyDollarValue | [pandas: `pandas.average_daily_dollar_value`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| AwesomeOscillator | [pandas-ta-classic: `pandas_ta_classic.ao`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| BarsSince | [pandas: `pandas.bars_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| BreakOfStructureChangeOfCharacter | [smartmoneyconcepts: `smartmoneyconcepts.break_of_structure_change_of_character`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| ChaikinMoneyFlow | [pandas-ta-classic: `pandas_ta_classic.cmf`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| ChaikinVolatility | [pandas-ta-classic: `pandas-ta-classic.chaikin_volatility`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| CloseToCloseSigma | [pandas: `pandas.close_to_close_sigma`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| Crossover | [pandas-ta-classic: `pandas-ta-classic.crossover`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| Crossunder | [pandas-ta-classic: `pandas-ta-classic.crossunder`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| CumulativeCount | [pandas: `pandas.cumulative_count`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| CumulativeMaximum | [Polars: `Polars.cumulative_maximum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 10.42× | 3.14× | 1.74× | 1.63× |
| CumulativeMinimum | [Polars: `Polars.cumulative_minimum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 10.89× | 2.95× | 1.73× | 1.60× |
| CumulativeProduct | [Polars: `Polars.cumulative_product`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 18.28× | 7.45× | 4.60× | 3.94× |
| CumulativeSum | [Polars: `Polars.cumulative_sum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 18.30× | 7.19× | 4.62× | 3.96× |
| CumulativeSumControlChart | [pandas: `pandas.cumulative_sum_control_chart`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| DecayLinear | [pandas: `pandas.decay_linear`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| DetrendedPriceOscillator | [pandas-ta-classic: `pandas_ta_classic.dpo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| DonchianChannels | [pandas-ta-classic: `pandas_ta_classic.donchian`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| Drawdown | [pandas: `pandas.drawdown`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| EaseOfMovement | [pandas-ta-classic: `pandas-ta-classic.ease_of_movement`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| EqualHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.equal_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| EvenBetterSinewave | [pandas-ta-classic: `pandas_ta_classic.ebsw`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| ExponentiallyWeightedCorrelation | [pandas: `pandas.ewm_corr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| ExponentiallyWeightedCovariance | [pandas: `pandas.ewm_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| ExponentiallyWeightedStandardDeviation | [pandas: `pandas.ewm_std`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| ExponentiallyWeightedVariance | [Polars: `Polars.ewm_var`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 9.29× | 4.18× | 3.35× | 2.96× |
| ExponentiallyWeightedSum | [pandas: `pandas.exponentially_weighted_sum`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| FairValueGap | [smartmoneyconcepts: `smartmoneyconcepts.fair_value_gap`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| Falling | [pandas: `pandas.falling`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| FibonacciRetracement | [pandas: `pandas.fib_retracement`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| FisherTransform | [pandas-ta-classic: `pandas_ta_classic.fisher`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| ForceIndex | [pandas-ta-classic: `pandas_ta_classic.efi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| FracDiff | [pandas: `pandas.frac_diff`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| FractalDimension | [pandas: `pandas.fractal_dimension`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| GapDown | [pandas: `pandas.gap_down`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| GapUp | [pandas: `pandas.gap_up`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| GarmanKlass | [pandas: `pandas.garman_klass`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| GarmanKlassYangZhang | [pandas: `pandas.garman_klass_yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| HedgeRatio | [pandas: `pandas.hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| HeikinAshi | [pandas-ta-classic: `pandas-ta-classic.heikin_ashi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| HigherHigh | [pandas: `pandas.higher_high`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| HighestSince | [pandas: `pandas.highest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| HullMovingAverage | [pandas-ta-classic: `pandas_ta_classic.hma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| Hurst | [pandas: `pandas.hurst`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| Ichimoku | [pandas-ta-classic: `pandas-ta-classic.ichimoku`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| InsideBar | [pandas: `pandas.inside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| JurikMovingAverage | [pandas-ta-classic: `pandas-ta-classic.jma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| KalmanHedgeRatio | [pandas: `pandas.kalman_hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| KeltnerChannels | [pandas-ta-classic: `pandas-ta-classic.keltner_channels`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| KnowSureThing | [pandas-ta-classic: `pandas-ta-classic.know_sure_thing`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| KlingerVolumeOscillator | [pandas-ta-classic: `pandas-ta-classic.kvo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| Lag | [pandas: `pandas.lag`](https://pandas.pydata.org/docs/reference/window.html) | 11.76× | 2.61× | 0.48× | 0.38× |
| LaguerreRelativeStrengthIndex | [pandas: `pandas.laguerre_rsi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| Liquidity | [smartmoneyconcepts: `smartmoneyconcepts.liquidity`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| LogReturn | [pandas-ta-classic: `pandas_ta_classic.log_return`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | 21.32× | 3.07× | 0.95× | 1.22× |
| LowerLow | [pandas: `pandas.lower_low`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| LowestSince | [pandas: `pandas.lowest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| MassIndex | [pandas-ta-classic: `pandas-ta-classic.mass_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| MathAbs | [NumPy: `numpy.abs`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.78× | 0.76× | 0.96× | 0.92× |
| MathAcosh | [NumPy: `numpy.arccosh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.94× | 1.00× | 1.01× | 0.98× |
| MathAsinh | [NumPy: `numpy.arcsinh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.82× | 0.98× | 0.97× | 1.01× |
| MathAtanh | [NumPy: `numpy.arctanh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 1.10× | 0.99× | 1.15× | 1.16× |
| MathCbrt | [NumPy: `numpy.cbrt`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.80× | 0.87× | 0.84× | 1.03× |
| MathCot | [NumPy: `numpy.tan reciprocal`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.96× | 1.04× | 1.04× | 1.04× |
| MathDegrees | [NumPy: `numpy.degrees`](https://numpy.org/doc/stable/reference/ufuncs.html) | 1.15× | 2.78× | 3.75× | 2.49× |
| MathLog1p | [NumPy: `numpy.log1p`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.91× | 1.02× | 1.07× | 1.00× |
| MathRadians | [NumPy: `numpy.radians`](https://numpy.org/doc/stable/reference/ufuncs.html) | 1.21× | 2.78× | 4.01× | 2.41× |
| McGinleyDynamic | [pandas-ta-classic: `pandas-ta-classic.mcginley`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| NegativeVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.negative_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| OpeningRange | [pandas: `pandas.opening_range`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| OrderBlock | [smartmoneyconcepts: `smartmoneyconcepts.order_block`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| OrnsteinUhlenbeckHalfLife | [pandas: `pandas.ornstein_uhlenbeck_half_life`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| OutsideBar | [pandas: `pandas.outside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| Parkinson | [pandas: `pandas.parkinson`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| PivotPoints | [pandas: `pandas.pivot_points`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| ParabolicMovingAverageStop | [pandas-ta-classic: `pandas-ta-classic.pmax`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| PositiveVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.positive_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| PremiumDiscount | [pandas: `pandas.premium_discount`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| PreviousHighLow | [smartmoneyconcepts: `smartmoneyconcepts.previous_high_low`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| Retracements | [smartmoneyconcepts: `smartmoneyconcepts.retracements`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| Rising | [pandas: `pandas.rising`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RelativeMomentumIndex | [pandas: `pandas.rmi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RogersSatchell | [pandas: `pandas.rogers_satchell`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollSpread | [pandas: `pandas.roll_spread`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingAlpha | [pandas: `pandas.rolling_alpha`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingAutocorr | [pandas: `pandas.rolling_autocorr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingCalmar | [pandas: `pandas.rolling_calmar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingCov | [pandas: `pandas.rolling_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingEntropy | [pandas: `pandas.rolling_entropy`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingInformationRatio | [pandas: `pandas.rolling_information_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingKurtosis | [pandas: `pandas.rolling_kurtosis`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingMedian | [pandas: `pandas.rolling_median`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingMode | [pandas: `pandas.rolling_mode`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingQuantile | [pandas: `pandas.rolling_quantile`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingRank | [pandas: `pandas.rolling_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingSharpe | [pandas: `pandas.rolling_sharpe`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingSkew | [pandas: `pandas.rolling_skew`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingSortino | [pandas: `pandas.rolling_sortino`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingVolumeWeightedAveragePrice | [pandas: `pandas.rolling_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingWinsorize | [pandas: `pandas.rolling_winsorize`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| RollingZScore | [pandas: `pandas.rolling_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| SchaffTrendCycle | [pandas-ta-classic: `pandas_ta_classic.stc`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| SessionVolumeLevels | [pandas: `pandas.session_volume_levels`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| Sessions | [smartmoneyconcepts: `smartmoneyconcepts.sessions`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| SignalDelay | [pandas: `pandas.signal_delay`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| SignedPower | [NumPy: `numpy.sign/numpy.abs/numpy.power`](https://numpy.org/doc/stable/reference/ufuncs.html) | 1.19× | 1.07× | 0.97× | 1.51× |
| SpreadZScore | [pandas: `pandas.spread_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| Squeeze | [pandas-ta-classic: `pandas-ta-classic.squeeze`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| SqueezePro | [pandas-ta-classic: `pandas-ta-classic.squeeze_pro`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| SmoothedTrendChannel | [pandas: `pandas.ssl_channel`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| Supertrend | [pandas-ta-classic: `pandas-ta-classic.supertrend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| SwingHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.swing_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — | — |
| TomDeMarkSequential | [pandas-ta-classic: `pandas-ta-classic.td_sequential`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| TimeSeriesRank | [pandas: `pandas.time_series_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| TrueStrengthIndex | [pandas-ta-classic: `pandas-ta-classic.true_strength_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| UlcerIndex | [pandas-ta-classic: `pandas-ta-classic.ulcer_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| ValueWhen | [pandas: `pandas.value_when`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| VariableIndexDynamicAverage | [pandas-ta-classic: `pandas-ta-classic.vidya`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| VolumePriceTrend | [pandas-ta-classic: `pandas-ta-classic.volume_price_trend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| VolumeWeightedMovingAverage | [pandas-ta-classic: `pandas_ta_classic.vwma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| Vortex | [pandas-ta-classic: `pandas-ta-classic.vortex`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |
| YangZhang | [pandas: `pandas.yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — | — |
| ZeroLagExponentialMovingAverage | [pandas-ta-classic: `pandas_ta_classic.zlma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — | — |

## 3. Warm up

Fresh independent states are constructed and fed the stated number of bars. Thread columns measure that many states concurrently.

### 1 bar

| Class | Reference | 1 thread | 5 threads | 10 threads |
|---|---|---:|---:|---:|
| AbsolutePriceOscillator | [TA-Lib: `APO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.55× | 1.91× |
| AccelerationBands | [TA-Lib: `ACCBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.53× | 2.09× | 2.18× |
| AccumulationDistribution | [TA-Lib: `AD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.89× | 1.75× | 1.84× |
| AccumulationDistributionOscillator | [TA-Lib: `ADOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.04× | 1.58× | 2.04× |
| Amihud | [pandas: `pandas.amihud`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| AnchoredVolumeWeightedAveragePrice | [pandas: `pandas.anchored_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ArnaudLegouxMovingAverage | [pandas-ta-classic: `pandas-ta-classic.arnaud_legoux_moving_average`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Aroon | [TA-Lib: `AROON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.21× | 2.77× | 1.93× |
| AroonOscillator | [TA-Lib: `AROONOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.56× | 1.87× |
| AverageDailyDollarValue | [pandas: `pandas.average_daily_dollar_value`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| AverageDirectionalIndex | [TA-Lib: `ADX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.44× | 1.93× | 2.20× |
| AverageDirectionalIndexRating | [TA-Lib: `ADXR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.42× | 1.66× | 1.80× |
| AveragePrice | [TA-Lib: `AVGPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.73× | 1.94× |
| AverageTrueRange | [TA-Lib: `ATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.57× | 1.66× | 1.92× |
| AwesomeOscillator | [pandas-ta-classic: `pandas_ta_classic.ao`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| BalanceOfPower | [TA-Lib: `BOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.06× | 1.53× | 1.83× |
| BarsSince | [pandas: `pandas.bars_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| BollingerBands | [TA-Lib: `BBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.42× | 2.07× | 2.52× |
| BreakOfStructureChangeOfCharacter | [smartmoneyconcepts: `smartmoneyconcepts.break_of_structure_change_of_character`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| CandleAbandonedBaby | [TA-Lib: `CDLABANDONEDBABY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.02× | 1.55× | 1.96× |
| CandleAdvanceBlock | [TA-Lib: `CDLADVANCEBLOCK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.97× | 1.41× | 1.74× |
| CandleBeltHold | [TA-Lib: `CDLBELTHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.12× | 1.85× | 1.86× |
| CandleBreakaway | [TA-Lib: `CDLBREAKAWAY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.72× | 1.47× | 1.84× |
| CandleClosingMarubozu | [TA-Lib: `CDLCLOSINGMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.32× | 1.64× | 1.83× |
| CandleConcealBabySwall | [TA-Lib: `CDLCONCEALBABYSWALL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.15× | 1.28× | 1.78× |
| CandleCounterAttack | [TA-Lib: `CDLCOUNTERATTACK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.09× | 1.67× | 1.83× |
| CandleDarkCloudCover | [TA-Lib: `CDLDARKCLOUDCOVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.25× | 1.69× | 1.91× |
| CandleDoji | [TA-Lib: `CDLDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.47× | 1.77× | 1.79× |
| CandleDojiStar | [TA-Lib: `CDLDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.97× | 1.45× | 1.86× |
| CandleDragonflyDoji | [TA-Lib: `CDLDRAGONFLYDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.20× | 1.61× | 1.91× |
| CandleEngulfing | [TA-Lib: `CDLENGULFING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.11× | 1.35× | 1.91× |
| CandleEveningDojiStar | [TA-Lib: `CDLEVENINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.65× | 1.69× | 1.95× |
| CandleEveningStar | [TA-Lib: `CDLEVENINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.94× | 1.88× | 1.91× |
| CandleGapSideSideWhite | [TA-Lib: `CDLGAPSIDESIDEWHITE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.44× | 1.48× | 1.81× |
| CandleGravestoneDoji | [TA-Lib: `CDLGRAVESTONEDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.95× | 1.66× | 1.81× |
| CandleHammer | [TA-Lib: `CDLHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.13× | 1.74× | 1.83× |
| CandleHangingMan | [TA-Lib: `CDLHANGINGMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.97× | 1.70× | 1.84× |
| CandleHarami | [TA-Lib: `CDLHARAMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.32× | 1.38× | 1.83× |
| CandleHaramiCross | [TA-Lib: `CDLHARAMICROSS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.27× | 1.39× | 1.82× |
| CandleHighWave | [TA-Lib: `CDLHIGHWAVE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.12× | 1.60× | 1.72× |
| CandleHikkake | [TA-Lib: `CDLHIKKAKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.06× | 1.58× | 1.84× |
| CandleHikkakeModified | [TA-Lib: `CDLHIKKAKEMOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.41× | 1.54× | 1.75× |
| CandleHomingPigeon | [TA-Lib: `CDLHOMINGPIGEON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.29× | 1.37× | 1.83× |
| CandleIdenticalThreeCrows | [TA-Lib: `CDLIDENTICAL3CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.48× | 1.69× | 1.88× |
| CandleInNeck | [TA-Lib: `CDLINNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.75× | 1.64× | 1.86× |
| CandleInvertedHammer | [TA-Lib: `CDLINVERTEDHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.22× | 1.80× |
| CandleKicking | [TA-Lib: `CDLKICKING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.15× | 1.28× | 1.82× |
| CandleKickingByLength | [TA-Lib: `CDLKICKINGBYLENGTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.99× | 1.68× | 1.81× |
| CandleLadderBottom | [TA-Lib: `CDLLADDERBOTTOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.25× | 1.70× | 1.82× |
| CandleLongLeggedDoji | [TA-Lib: `CDLLONGLEGGEDDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.14× | 1.42× | 1.74× |
| CandleLongLine | [TA-Lib: `CDLLONGLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.15× | 1.43× | 1.85× |
| CandleMarubozu | [TA-Lib: `CDLMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.10× | 1.38× | 1.76× |
| CandleMatHold | [TA-Lib: `CDLMATHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.50× | 1.93× | 1.88× |
| CandleMatchingLow | [TA-Lib: `CDLMATCHINGLOW`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.43× | 1.61× | 1.72× |
| CandleMorningDojiStar | [TA-Lib: `CDLMORNINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 1.51× | 1.95× |
| CandleMorningStar | [TA-Lib: `CDLMORNINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.21× | 1.70× | 1.98× |
| CandleOnNeck | [TA-Lib: `CDLONNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.11× | 1.30× | 1.77× |
| CandlePiercing | [TA-Lib: `CDLPIERCING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.10× | 1.58× | 1.85× |
| CandleRickshawman | [TA-Lib: `CDLRICKSHAWMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.35× | 1.93× | 1.85× |
| CandleRiseFallThreeMethods | [TA-Lib: `CDLRISEFALL3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 2.00× | 1.82× |
| CandleSeparatingLines | [TA-Lib: `CDLSEPARATINGLINES`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.85× | 1.43× | 1.76× |
| CandleShootingStar | [TA-Lib: `CDLSHOOTINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.08× | 1.62× | 1.77× |
| CandleShortLine | [TA-Lib: `CDLSHORTLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.22× | 1.61× | 1.82× |
| CandleSpinningTop | [TA-Lib: `CDLSPINNINGTOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.11× | 1.55× | 1.79× |
| CandleStalledPattern | [TA-Lib: `CDLSTALLEDPATTERN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.10× | 1.79× | 1.72× |
| CandleStickSandwich | [TA-Lib: `CDLSTICKSANDWICH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.05× | 1.86× | 1.44× |
| CandleTakuri | [TA-Lib: `CDLTAKURI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.20× | 1.53× | 1.83× |
| CandleTasukiGap | [TA-Lib: `CDLTASUKIGAP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.88× | 1.79× | 1.84× |
| CandleThreeBlackCrows | [TA-Lib: `CDL3BLACKCROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.07× | 1.58× | 1.78× |
| CandleThreeInside | [TA-Lib: `CDL3INSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.04× | 1.35× | 1.91× |
| CandleThreeLineStrike | [TA-Lib: `CDL3LINESTRIKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.68× | 1.54× |
| CandleThreeOutside | [TA-Lib: `CDL3OUTSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.28× | 1.57× | 1.85× |
| CandleThreeStarsInSouth | [TA-Lib: `CDL3STARSINSOUTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.80× | 1.81× |
| CandleThreeWhiteSoldiers | [TA-Lib: `CDL3WHITESOLDIERS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.03× | 1.59× | 1.77× |
| CandleThrusting | [TA-Lib: `CDLTHRUSTING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.11× | 1.46× | 1.86× |
| CandleTriStar | [TA-Lib: `CDLTRISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.17× | 1.53× | 1.89× |
| CandleTwoCrows | [TA-Lib: `CDL2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.86× | 1.53× | 1.78× |
| CandleUniqueThreeRiver | [TA-Lib: `CDLUNIQUE3RIVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.39× | 1.57× | 1.78× |
| CandleUpDownSideGapThreeMethods | [TA-Lib: `CDLXSIDEGAP3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.13× | 1.30× | 1.81× |
| CandleUpsideGapTwoCrows | [TA-Lib: `CDLUPSIDEGAP2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.13× | 1.57× | 1.77× |
| ChaikinMoneyFlow | [pandas-ta-classic: `pandas_ta_classic.cmf`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ChaikinVolatility | [pandas-ta-classic: `pandas-ta-classic.chaikin_volatility`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ChandeMomentumOscillator | [TA-Lib: `CMO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.43× | 1.83× | 2.19× |
| CloseToCloseSigma | [pandas: `pandas.close_to_close_sigma`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| CommodityChannelIndex | [TA-Lib: `CCI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.89× | 1.95× | 1.92× |
| Crossover | [pandas-ta-classic: `pandas-ta-classic.crossover`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Crossunder | [pandas-ta-classic: `pandas-ta-classic.crossunder`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| CumulativeCount | [pandas: `pandas.cumulative_count`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| CumulativeMaximum | [Polars: `Polars.cumulative_maximum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 4.43× | 2.59× | 2.60× |
| CumulativeMinimum | [Polars: `Polars.cumulative_minimum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.22× | 2.70× | 2.80× |
| CumulativeProduct | [Polars: `Polars.cumulative_product`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 1.17× | 2.16× | 2.71× |
| CumulativeSum | [Polars: `Polars.cumulative_sum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 1.91× | 2.89× | 2.91× |
| CumulativeSumControlChart | [pandas: `pandas.cumulative_sum_control_chart`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| DecayLinear | [pandas: `pandas.decay_linear`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| DetrendedPriceOscillator | [pandas-ta-classic: `pandas_ta_classic.dpo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| DirectionalMovementIndex | [TA-Lib: `DX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.94× | 1.73× | 1.99× |
| DonchianChannels | [pandas-ta-classic: `pandas_ta_classic.donchian`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| DoubleExponentialMovingAverage | [TA-Lib: `DEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.66× | 1.81× |
| Drawdown | [pandas: `pandas.drawdown`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| EaseOfMovement | [pandas-ta-classic: `pandas-ta-classic.ease_of_movement`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| EqualHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.equal_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| EvenBetterSinewave | [pandas-ta-classic: `pandas_ta_classic.ebsw`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ExponentialMovingAverage | [TA-Lib: `EMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.34× | 1.48× | 1.86× |
| ExponentiallyWeightedCorrelation | [pandas: `pandas.ewm_corr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedCovariance | [pandas: `pandas.ewm_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedStandardDeviation | [pandas: `pandas.ewm_std`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedSum | [pandas: `pandas.exponentially_weighted_sum`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedVariance | [Polars: `Polars.ewm_var`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 2.55× | 2.64× | 2.73× |
| FairValueGap | [smartmoneyconcepts: `smartmoneyconcepts.fair_value_gap`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| Falling | [pandas: `pandas.falling`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FastStochasticOscillator | [TA-Lib: `STOCHF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 5.33× | 0.91× | 0.94× |
| FibonacciRetracement | [pandas: `pandas.fib_retracement`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FisherTransform | [pandas-ta-classic: `pandas_ta_classic.fisher`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ForceIndex | [pandas-ta-classic: `pandas_ta_classic.efi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| FracDiff | [pandas: `pandas.frac_diff`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FractalDimension | [pandas: `pandas.fractal_dimension`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GapDown | [pandas: `pandas.gap_down`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GapUp | [pandas: `pandas.gap_up`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GarmanKlass | [pandas: `pandas.garman_klass`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GarmanKlassYangZhang | [pandas: `pandas.garman_klass_yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HedgeRatio | [pandas: `pandas.hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HeikinAshi | [pandas-ta-classic: `pandas-ta-classic.heikin_ashi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| HigherHigh | [pandas: `pandas.higher_high`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HighestSince | [pandas: `pandas.highest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HilbertTransformDominantCyclePeriod | [TA-Lib: `HT_DCPERIOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.36× | 1.74× | 2.07× |
| HilbertTransformDominantCyclePhase | [TA-Lib: `HT_DCPHASE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.03× | 1.62× | 2.01× |
| HilbertTransformPhasor | [TA-Lib: `HT_PHASOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.19× | 1.90× | 1.94× |
| HilbertTransformSineWave | [TA-Lib: `HT_SINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 1.93× | 2.14× |
| HilbertTransformTrendMode | [TA-Lib: `HT_TRENDMODE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.18× | 1.81× | 1.96× |
| HilbertTransformTrendline | [TA-Lib: `HT_TRENDLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.13× | 1.62× | 2.03× |
| HullMovingAverage | [pandas-ta-classic: `pandas_ta_classic.hma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Hurst | [pandas: `pandas.hurst`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Ichimoku | [pandas-ta-classic: `pandas-ta-classic.ichimoku`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| InsideBar | [pandas: `pandas.inside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| IntradayMomentumIndex | [TA-Lib: `IMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.17× | 2.09× | 2.12× |
| JurikMovingAverage | [pandas-ta-classic: `pandas-ta-classic.jma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KalmanHedgeRatio | [pandas: `pandas.kalman_hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| KaufmanAdaptiveMovingAverage | [TA-Lib: `KAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.42× | 1.61× | 2.04× |
| KeltnerChannels | [pandas-ta-classic: `pandas-ta-classic.keltner_channels`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KlingerVolumeOscillator | [pandas-ta-classic: `pandas-ta-classic.kvo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KnowSureThing | [pandas-ta-classic: `pandas-ta-classic.know_sure_thing`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Lag | [pandas: `pandas.lag`](https://pandas.pydata.org/docs/reference/window.html) | 1.45× | 2.64× | 2.92× |
| LaguerreRelativeStrengthIndex | [pandas: `pandas.laguerre_rsi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Liquidity | [smartmoneyconcepts: `smartmoneyconcepts.liquidity`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| LogReturn | [pandas-ta-classic: `pandas_ta_classic.log_return`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | 3.53× | 3.71× | 5.20× |
| LowerLow | [pandas: `pandas.lower_low`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| LowestSince | [pandas: `pandas.lowest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| MassIndex | [pandas-ta-classic: `pandas-ta-classic.mass_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| MathAbs | [NumPy: `numpy.abs`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.74× | 0.83× | 0.80× |
| MathAcos | [TA-Lib: `ACOS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.98× | 1.51× | 1.96× |
| MathAcosh | [NumPy: `numpy.arccosh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.55× | 0.93× | 0.86× |
| MathAdd | [TA-Lib: `ADD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.88× | 1.69× | 1.90× |
| MathAsin | [TA-Lib: `ASIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.47× | 1.92× | 1.90× |
| MathAsinh | [NumPy: `numpy.arcsinh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.81× | 0.76× | 0.89× |
| MathAtan | [TA-Lib: `ATAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.34× | 1.97× | 2.04× |
| MathAtanh | [NumPy: `numpy.arctanh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.66× | 0.74× | 0.88× |
| MathCbrt | [NumPy: `numpy.cbrt`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.43× | 0.63× | 0.84× |
| MathCeil | [TA-Lib: `CEIL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.15× | 1.73× | 1.87× |
| MathCos | [TA-Lib: `COS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.08× | 1.76× | 1.99× |
| MathCosh | [TA-Lib: `COSH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.80× | 1.97× | 2.07× |
| MathCot | [NumPy: `numpy.tan reciprocal`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.78× | 0.83× | 0.88× |
| MathDegrees | [NumPy: `numpy.degrees`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.69× | 1.03× | 1.22× |
| MathDivide | [TA-Lib: `DIV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.03× | 1.53× | 1.81× |
| MathExp | [TA-Lib: `EXP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.24× | 1.36× | 2.12× |
| MathFloor | [TA-Lib: `FLOOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.92× | 1.32× | 1.97× |
| MathLn | [TA-Lib: `LN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.15× | 2.01× | 1.93× |
| MathLog10 | [TA-Lib: `LOG10`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.51× | 1.91× | 1.96× |
| MathLog1p | [NumPy: `numpy.log1p`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.59× | 0.84× | 0.82× |
| MathMultiply | [TA-Lib: `MULT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.46× | 1.12× | 1.89× |
| MathRadians | [NumPy: `numpy.radians`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.79× | 0.68× | 0.82× |
| MathSin | [TA-Lib: `SIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.05× | 1.64× | 1.90× |
| MathSinh | [TA-Lib: `SINH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.47× | 1.50× | 1.97× |
| MathSqrt | [TA-Lib: `SQRT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.37× | 1.48× | 2.06× |
| MathSubtract | [TA-Lib: `SUB`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.66× | 1.56× | 1.94× |
| MathTan | [TA-Lib: `TAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.44× | 1.98× | 1.82× |
| MathTanh | [TA-Lib: `TANH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.07× | 1.69× | 1.98× |
| McGinleyDynamic | [pandas-ta-classic: `pandas-ta-classic.mcginley`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| MedianPrice | [TA-Lib: `MEDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.42× | 1.69× | 1.92× |
| MesaAdaptiveMovingAverage | [TA-Lib: `MAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.20× | 2.14× | 2.27× |
| MinusDirectionalIndicator | [TA-Lib: `MINUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.57× | 1.72× | 1.95× |
| MinusDirectionalMovement | [TA-Lib: `MINUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.24× | 1.57× | 1.88× |
| Momentum | [TA-Lib: `MOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.08× | 1.86× | 1.96× |
| MoneyFlowIndex | [TA-Lib: `MFI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 1.56× | 1.84× |
| MovingAverage | [TA-Lib: `MA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.56× | 1.96× | 2.31× |
| MovingAverageConvergenceDivergence | [TA-Lib: `MACD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.43× | 2.28× |
| MovingAverageConvergenceDivergenceExtended | [TA-Lib: `MACDEXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 2.29× | 2.19× |
| MovingAverageConvergenceDivergenceFixed | [TA-Lib: `MACDFIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 1.84× | 2.44× |
| NegativeVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.negative_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| NormalizedAverageTrueRange | [TA-Lib: `NATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.13× | 1.97× | 1.96× |
| OnBalanceVolume | [TA-Lib: `OBV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.12× | 1.19× | 1.96× |
| OpeningRange | [pandas: `pandas.opening_range`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| OrderBlock | [smartmoneyconcepts: `smartmoneyconcepts.order_block`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| OrnsteinUhlenbeckHalfLife | [pandas: `pandas.ornstein_uhlenbeck_half_life`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| OutsideBar | [pandas: `pandas.outside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ParabolicMovingAverageStop | [pandas-ta-classic: `pandas-ta-classic.pmax`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ParabolicSar | [TA-Lib: `SAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.40× | 2.04× | 2.14× |
| ParabolicSarExtended | [TA-Lib: `SAREXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 2.18× | 2.64× |
| Parkinson | [pandas: `pandas.parkinson`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PercentagePriceOscillator | [TA-Lib: `PPO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.13× | 2.00× | 1.91× |
| PivotPoints | [pandas: `pandas.pivot_points`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PlusDirectionalIndicator | [TA-Lib: `PLUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.45× | 1.75× | 1.97× |
| PlusDirectionalMovement | [TA-Lib: `PLUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.11× | 1.82× | 2.18× |
| PositiveVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.positive_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| PremiumDiscount | [pandas: `pandas.premium_discount`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PreviousHighLow | [smartmoneyconcepts: `smartmoneyconcepts.previous_high_low`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| RateOfChange | [TA-Lib: `ROC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.25× | 1.80× | 2.01× |
| RateOfChangePercent | [TA-Lib: `ROCP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.49× | 1.53× | 1.88× |
| RateOfChangeRatio | [TA-Lib: `ROCR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.13× | 1.95× | 1.99× |
| RateOfChangeRatioPercent | [TA-Lib: `ROCR100`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.95× | 2.00× | 1.95× |
| RelativeMomentumIndex | [pandas: `pandas.rmi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RelativeStrengthIndex | [TA-Lib: `RSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.92× | 2.13× | 1.92× |
| Retracements | [smartmoneyconcepts: `smartmoneyconcepts.retracements`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| Rising | [pandas: `pandas.rising`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RogersSatchell | [pandas: `pandas.rogers_satchell`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollSpread | [pandas: `pandas.roll_spread`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingAlpha | [pandas: `pandas.rolling_alpha`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingArgmax | [TA-Lib: `MAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.96× | 2.02× | 1.96× |
| RollingArgmin | [TA-Lib: `MININDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.22× | 1.94× | 1.99× |
| RollingAutocorr | [pandas: `pandas.rolling_autocorr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingAverageDeviation | [TA-Lib: `AVGDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.41× | 1.63× | 2.03× |
| RollingBeta | [TA-Lib: `BETA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.59× | 1.62× | 1.79× |
| RollingCalmar | [pandas: `pandas.rolling_calmar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingCorrelation | [TA-Lib: `CORREL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.10× | 2.02× | 1.98× |
| RollingCov | [pandas: `pandas.rolling_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingEntropy | [pandas: `pandas.rolling_entropy`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingInformationRatio | [pandas: `pandas.rolling_information_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingKurtosis | [pandas: `pandas.rolling_kurtosis`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingLinearRegression | [TA-Lib: `LINEARREG`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.43× | 1.45× | 2.00× |
| RollingLinearRegressionAngle | [TA-Lib: `LINEARREG_ANGLE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 1.89× | 1.86× |
| RollingLinearRegressionIntercept | [TA-Lib: `LINEARREG_INTERCEPT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.50× | 1.80× | 2.02× |
| RollingLinearRegressionSlope | [TA-Lib: `LINEARREG_SLOPE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 1.65× | 1.83× |
| RollingMax | [TA-Lib: `MAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.81× | 2.19× |
| RollingMedian | [pandas: `pandas.rolling_median`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingMidpoint | [TA-Lib: `MIDPOINT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.56× | 1.99× | 2.15× |
| RollingMidprice | [TA-Lib: `MIDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.82× | 1.88× |
| RollingMin | [TA-Lib: `MIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.94× | 1.93× | 2.07× |
| RollingMinMax | [TA-Lib: `MINMAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.31× | 1.79× | 2.11× |
| RollingMinMaxIndex | [TA-Lib: `MINMAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.94× | 2.20× |
| RollingMode | [pandas: `pandas.rolling_mode`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingQuantile | [pandas: `pandas.rolling_quantile`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingRank | [pandas: `pandas.rolling_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSharpe | [pandas: `pandas.rolling_sharpe`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSkew | [pandas: `pandas.rolling_skew`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSortino | [pandas: `pandas.rolling_sortino`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingStandardDeviation | [TA-Lib: `STDDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.00× | 1.84× | 1.90× |
| RollingSum | [TA-Lib: `SUM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.93× | 1.92× |
| RollingTimeSeriesForecast | [TA-Lib: `TSF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.29× | 1.62× | 2.01× |
| RollingVariance | [TA-Lib: `VAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.08× | 1.74× | 2.15× |
| RollingVolumeWeightedAveragePrice | [pandas: `pandas.rolling_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingWinsorize | [pandas: `pandas.rolling_winsorize`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingZScore | [pandas: `pandas.rolling_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SchaffTrendCycle | [pandas-ta-classic: `pandas_ta_classic.stc`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SessionVolumeLevels | [pandas: `pandas.session_volume_levels`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Sessions | [smartmoneyconcepts: `smartmoneyconcepts.sessions`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| SignalDelay | [pandas: `pandas.signal_delay`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SignedPower | [NumPy: `numpy.sign/numpy.abs/numpy.power`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.75× | 0.87× | 0.92× |
| SimpleMovingAverage | [TA-Lib: `SMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.36× | 1.71× | 1.96× |
| SmoothedTrendChannel | [pandas: `pandas.ssl_channel`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SpreadZScore | [pandas: `pandas.spread_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Squeeze | [pandas-ta-classic: `pandas-ta-classic.squeeze`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SqueezePro | [pandas-ta-classic: `pandas-ta-classic.squeeze_pro`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| StochasticOscillator | [TA-Lib: `STOCH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.46× | 1.81× | 2.29× |
| StochasticRelativeStrengthIndex | [TA-Lib: `STOCHRSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 1.94× | 2.29× |
| Supertrend | [pandas-ta-classic: `pandas-ta-classic.supertrend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SwingHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.swing_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| TimeSeriesRank | [pandas: `pandas.time_series_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| TomDeMarkSequential | [pandas-ta-classic: `pandas-ta-classic.td_sequential`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| TriangularMovingAverage | [TA-Lib: `TRIMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.40× | 1.91× | 2.00× |
| TripleExponentialAverage | [TA-Lib: `T3`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.57× | 1.69× | 2.11× |
| TripleExponentialMovingAverage | [TA-Lib: `TEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.18× | 1.69× | 2.04× |
| TripleExponentialRateOfChange | [TA-Lib: `TRIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.51× | 1.57× | 1.93× |
| TrueRange | [TA-Lib: `TRANGE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 1.90× | 1.91× |
| TrueStrengthIndex | [pandas-ta-classic: `pandas-ta-classic.true_strength_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| TypicalPrice | [TA-Lib: `TYPPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 1.90× | 1.91× |
| UlcerIndex | [pandas-ta-classic: `pandas-ta-classic.ulcer_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| UltimateOscillator | [TA-Lib: `ULTOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.38× | 1.91× | 1.94× |
| ValueWhen | [pandas: `pandas.value_when`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| VariableIndexDynamicAverage | [pandas-ta-classic: `pandas-ta-classic.vidya`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| VariablePeriodMovingAverage | [TA-Lib: `MAVP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.42× | 1.50× | 1.96× |
| VolumePriceTrend | [pandas-ta-classic: `pandas-ta-classic.volume_price_trend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| VolumeWeightedMovingAverage | [pandas-ta-classic: `pandas_ta_classic.vwma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Vortex | [pandas-ta-classic: `pandas-ta-classic.vortex`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| WeightedClose | [TA-Lib: `WCLPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.07× | 1.60× | 1.89× |
| WeightedMovingAverage | [TA-Lib: `WMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.93× | 1.76× | 2.12× |
| WilliamsPercentR | [TA-Lib: `WILLR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.98× | 1.60× | 2.02× |
| YangZhang | [pandas: `pandas.yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ZeroLagExponentialMovingAverage | [pandas-ta-classic: `pandas_ta_classic.zlma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |

### 10 bars

| Class | Reference | 1 thread | 5 threads | 10 threads |
|---|---|---:|---:|---:|
| AbsolutePriceOscillator | [TA-Lib: `APO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 2.02× | 1.95× |
| AccelerationBands | [TA-Lib: `ACCBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.09× | 2.26× | 2.30× |
| AccumulationDistribution | [TA-Lib: `AD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.74× | 1.97× |
| AccumulationDistributionOscillator | [TA-Lib: `ADOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.90× | 2.00× | 1.94× |
| Amihud | [pandas: `pandas.amihud`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| AnchoredVolumeWeightedAveragePrice | [pandas: `pandas.anchored_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ArnaudLegouxMovingAverage | [pandas-ta-classic: `pandas-ta-classic.arnaud_legoux_moving_average`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Aroon | [TA-Lib: `AROON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.90× | 2.08× | 1.84× |
| AroonOscillator | [TA-Lib: `AROONOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.94× | 2.02× |
| AverageDailyDollarValue | [pandas: `pandas.average_daily_dollar_value`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| AverageDirectionalIndex | [TA-Lib: `ADX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 2.15× | 1.96× |
| AverageDirectionalIndexRating | [TA-Lib: `ADXR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.91× | 2.09× | 1.98× |
| AveragePrice | [TA-Lib: `AVGPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.83× | 1.89× |
| AverageTrueRange | [TA-Lib: `ATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.97× | 1.87× | 1.92× |
| AwesomeOscillator | [pandas-ta-classic: `pandas_ta_classic.ao`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| BalanceOfPower | [TA-Lib: `BOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.94× | 1.77× |
| BarsSince | [pandas: `pandas.bars_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| BollingerBands | [TA-Lib: `BBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.24× | 2.38× | 2.52× |
| BreakOfStructureChangeOfCharacter | [smartmoneyconcepts: `smartmoneyconcepts.break_of_structure_change_of_character`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| CandleAbandonedBaby | [TA-Lib: `CDLABANDONEDBABY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.90× | 1.88× |
| CandleAdvanceBlock | [TA-Lib: `CDLADVANCEBLOCK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.82× | 1.80× |
| CandleBeltHold | [TA-Lib: `CDLBELTHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.72× | 1.85× |
| CandleBreakaway | [TA-Lib: `CDLBREAKAWAY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.88× | 1.75× |
| CandleClosingMarubozu | [TA-Lib: `CDLCLOSINGMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.86× | 1.78× |
| CandleConcealBabySwall | [TA-Lib: `CDLCONCEALBABYSWALL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.75× | 1.83× |
| CandleCounterAttack | [TA-Lib: `CDLCOUNTERATTACK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.79× | 1.77× |
| CandleDarkCloudCover | [TA-Lib: `CDLDARKCLOUDCOVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 1.85× | 1.90× |
| CandleDoji | [TA-Lib: `CDLDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.93× | 1.76× |
| CandleDojiStar | [TA-Lib: `CDLDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.75× | 1.88× |
| CandleDragonflyDoji | [TA-Lib: `CDLDRAGONFLYDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.74× | 1.83× |
| CandleEngulfing | [TA-Lib: `CDLENGULFING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.83× | 1.87× |
| CandleEveningDojiStar | [TA-Lib: `CDLEVENINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.92× | 1.91× |
| CandleEveningStar | [TA-Lib: `CDLEVENINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.88× | 1.87× |
| CandleGapSideSideWhite | [TA-Lib: `CDLGAPSIDESIDEWHITE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.86× | 1.81× |
| CandleGravestoneDoji | [TA-Lib: `CDLGRAVESTONEDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.92× | 1.80× |
| CandleHammer | [TA-Lib: `CDLHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.82× | 1.92× |
| CandleHangingMan | [TA-Lib: `CDLHANGINGMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.78× | 1.84× |
| CandleHarami | [TA-Lib: `CDLHARAMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.81× | 1.85× |
| CandleHaramiCross | [TA-Lib: `CDLHARAMICROSS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.90× | 1.82× |
| CandleHighWave | [TA-Lib: `CDLHIGHWAVE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.08× | 1.76× | 1.74× |
| CandleHikkake | [TA-Lib: `CDLHIKKAKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.88× | 1.87× |
| CandleHikkakeModified | [TA-Lib: `CDLHIKKAKEMOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.72× | 1.85× |
| CandleHomingPigeon | [TA-Lib: `CDLHOMINGPIGEON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 1.86× | 1.81× |
| CandleIdenticalThreeCrows | [TA-Lib: `CDLIDENTICAL3CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.81× | 1.83× |
| CandleInNeck | [TA-Lib: `CDLINNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.81× | 1.70× |
| CandleInvertedHammer | [TA-Lib: `CDLINVERTEDHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 1.83× | 1.78× |
| CandleKicking | [TA-Lib: `CDLKICKING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.85× | 1.81× |
| CandleKickingByLength | [TA-Lib: `CDLKICKINGBYLENGTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.81× | 1.91× |
| CandleLadderBottom | [TA-Lib: `CDLLADDERBOTTOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.76× | 1.81× |
| CandleLongLeggedDoji | [TA-Lib: `CDLLONGLEGGEDDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.83× | 1.82× |
| CandleLongLine | [TA-Lib: `CDLLONGLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.52× | 1.90× | 1.47× |
| CandleMarubozu | [TA-Lib: `CDLMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.75× | 1.80× |
| CandleMatHold | [TA-Lib: `CDLMATHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 2.03× | 1.84× |
| CandleMatchingLow | [TA-Lib: `CDLMATCHINGLOW`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.82× | 1.92× |
| CandleMorningDojiStar | [TA-Lib: `CDLMORNINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.84× | 1.91× |
| CandleMorningStar | [TA-Lib: `CDLMORNINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 1.90× | 1.86× |
| CandleOnNeck | [TA-Lib: `CDLONNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.79× | 1.83× |
| CandlePiercing | [TA-Lib: `CDLPIERCING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.65× | 1.82× | 1.82× |
| CandleRickshawman | [TA-Lib: `CDLRICKSHAWMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.79× | 1.78× |
| CandleRiseFallThreeMethods | [TA-Lib: `CDLRISEFALL3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 1.77× | 1.87× |
| CandleSeparatingLines | [TA-Lib: `CDLSEPARATINGLINES`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 1.81× | 1.80× |
| CandleShootingStar | [TA-Lib: `CDLSHOOTINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 1.84× | 1.82× |
| CandleShortLine | [TA-Lib: `CDLSHORTLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.91× | 1.80× |
| CandleSpinningTop | [TA-Lib: `CDLSPINNINGTOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.87× | 1.80× |
| CandleStalledPattern | [TA-Lib: `CDLSTALLEDPATTERN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 1.88× | 1.77× |
| CandleStickSandwich | [TA-Lib: `CDLSTICKSANDWICH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.90× | 1.83× |
| CandleTakuri | [TA-Lib: `CDLTAKURI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.71× | 1.84× |
| CandleTasukiGap | [TA-Lib: `CDLTASUKIGAP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.84× | 1.77× |
| CandleThreeBlackCrows | [TA-Lib: `CDL3BLACKCROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.77× | 1.70× |
| CandleThreeInside | [TA-Lib: `CDL3INSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.79× | 1.83× |
| CandleThreeLineStrike | [TA-Lib: `CDL3LINESTRIKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.71× | 1.94× |
| CandleThreeOutside | [TA-Lib: `CDL3OUTSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.78× | 1.71× |
| CandleThreeStarsInSouth | [TA-Lib: `CDL3STARSINSOUTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.65× | 1.74× | 1.77× |
| CandleThreeWhiteSoldiers | [TA-Lib: `CDL3WHITESOLDIERS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.82× | 1.82× |
| CandleThrusting | [TA-Lib: `CDLTHRUSTING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.91× | 1.83× |
| CandleTriStar | [TA-Lib: `CDLTRISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 1.80× | 1.84× |
| CandleTwoCrows | [TA-Lib: `CDL2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.90× | 1.76× |
| CandleUniqueThreeRiver | [TA-Lib: `CDLUNIQUE3RIVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.80× | 1.83× |
| CandleUpDownSideGapThreeMethods | [TA-Lib: `CDLXSIDEGAP3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.76× | 1.92× |
| CandleUpsideGapTwoCrows | [TA-Lib: `CDLUPSIDEGAP2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.81× | 1.87× |
| ChaikinMoneyFlow | [pandas-ta-classic: `pandas_ta_classic.cmf`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ChaikinVolatility | [pandas-ta-classic: `pandas-ta-classic.chaikin_volatility`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ChandeMomentumOscillator | [TA-Lib: `CMO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.54× | 2.03× | 2.09× |
| CloseToCloseSigma | [pandas: `pandas.close_to_close_sigma`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| CommodityChannelIndex | [TA-Lib: `CCI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.93× | 1.78× |
| Crossover | [pandas-ta-classic: `pandas-ta-classic.crossover`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Crossunder | [pandas-ta-classic: `pandas-ta-classic.crossunder`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| CumulativeCount | [pandas: `pandas.cumulative_count`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| CumulativeMaximum | [Polars: `Polars.cumulative_maximum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.15× | 2.46× | 2.70× |
| CumulativeMinimum | [Polars: `Polars.cumulative_minimum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.15× | 2.65× | 2.67× |
| CumulativeProduct | [Polars: `Polars.cumulative_product`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.03× | 2.49× | 3.19× |
| CumulativeSum | [Polars: `Polars.cumulative_sum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.02× | 2.47× | 2.67× |
| CumulativeSumControlChart | [pandas: `pandas.cumulative_sum_control_chart`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| DecayLinear | [pandas: `pandas.decay_linear`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| DetrendedPriceOscillator | [pandas-ta-classic: `pandas_ta_classic.dpo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| DirectionalMovementIndex | [TA-Lib: `DX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.98× | 2.04× | 1.89× |
| DonchianChannels | [pandas-ta-classic: `pandas_ta_classic.donchian`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| DoubleExponentialMovingAverage | [TA-Lib: `DEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.88× | 1.99× |
| Drawdown | [pandas: `pandas.drawdown`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| EaseOfMovement | [pandas-ta-classic: `pandas-ta-classic.ease_of_movement`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| EqualHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.equal_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| EvenBetterSinewave | [pandas-ta-classic: `pandas_ta_classic.ebsw`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ExponentialMovingAverage | [TA-Lib: `EMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 2.08× | 2.00× |
| ExponentiallyWeightedCorrelation | [pandas: `pandas.ewm_corr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedCovariance | [pandas: `pandas.ewm_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedStandardDeviation | [pandas: `pandas.ewm_std`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedSum | [pandas: `pandas.exponentially_weighted_sum`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedVariance | [Polars: `Polars.ewm_var`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.16× | 2.47× | 2.77× |
| FairValueGap | [smartmoneyconcepts: `smartmoneyconcepts.fair_value_gap`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| Falling | [pandas: `pandas.falling`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FastStochasticOscillator | [TA-Lib: `STOCHF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 0.17× | 4.66× | 1.75× |
| FibonacciRetracement | [pandas: `pandas.fib_retracement`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FisherTransform | [pandas-ta-classic: `pandas_ta_classic.fisher`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ForceIndex | [pandas-ta-classic: `pandas_ta_classic.efi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| FracDiff | [pandas: `pandas.frac_diff`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FractalDimension | [pandas: `pandas.fractal_dimension`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GapDown | [pandas: `pandas.gap_down`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GapUp | [pandas: `pandas.gap_up`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GarmanKlass | [pandas: `pandas.garman_klass`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GarmanKlassYangZhang | [pandas: `pandas.garman_klass_yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HedgeRatio | [pandas: `pandas.hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HeikinAshi | [pandas-ta-classic: `pandas-ta-classic.heikin_ashi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| HigherHigh | [pandas: `pandas.higher_high`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HighestSince | [pandas: `pandas.highest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HilbertTransformDominantCyclePeriod | [TA-Lib: `HT_DCPERIOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 1.87× | 1.98× |
| HilbertTransformDominantCyclePhase | [TA-Lib: `HT_DCPHASE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.87× | 1.97× |
| HilbertTransformPhasor | [TA-Lib: `HT_PHASOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.53× | 1.93× | 2.10× |
| HilbertTransformSineWave | [TA-Lib: `HT_SINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 2.18× | 1.91× |
| HilbertTransformTrendMode | [TA-Lib: `HT_TRENDMODE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 1.91× | 2.01× |
| HilbertTransformTrendline | [TA-Lib: `HT_TRENDLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 2.03× | 1.96× |
| HullMovingAverage | [pandas-ta-classic: `pandas_ta_classic.hma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Hurst | [pandas: `pandas.hurst`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Ichimoku | [pandas-ta-classic: `pandas-ta-classic.ichimoku`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| InsideBar | [pandas: `pandas.inside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| IntradayMomentumIndex | [TA-Lib: `IMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.05× | 2.16× | 2.02× |
| JurikMovingAverage | [pandas-ta-classic: `pandas-ta-classic.jma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KalmanHedgeRatio | [pandas: `pandas.kalman_hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| KaufmanAdaptiveMovingAverage | [TA-Lib: `KAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.90× | 1.87× |
| KeltnerChannels | [pandas-ta-classic: `pandas-ta-classic.keltner_channels`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KlingerVolumeOscillator | [pandas-ta-classic: `pandas-ta-classic.kvo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KnowSureThing | [pandas-ta-classic: `pandas-ta-classic.know_sure_thing`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Lag | [pandas: `pandas.lag`](https://pandas.pydata.org/docs/reference/window.html) | 2.63× | 2.84× | 2.95× |
| LaguerreRelativeStrengthIndex | [pandas: `pandas.laguerre_rsi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Liquidity | [smartmoneyconcepts: `smartmoneyconcepts.liquidity`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| LogReturn | [pandas-ta-classic: `pandas_ta_classic.log_return`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | 4.52× | 5.31× | 5.06× |
| LowerLow | [pandas: `pandas.lower_low`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| LowestSince | [pandas: `pandas.lowest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| MassIndex | [pandas-ta-classic: `pandas-ta-classic.mass_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| MathAbs | [NumPy: `numpy.abs`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.89× | 0.98× | 0.84× |
| MathAcos | [TA-Lib: `ACOS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.78× | 2.08× |
| MathAcosh | [NumPy: `numpy.arccosh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.87× | 0.90× | 0.82× |
| MathAdd | [TA-Lib: `ADD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.93× | 1.84× |
| MathAsin | [TA-Lib: `ASIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 1.91× | 1.96× |
| MathAsinh | [NumPy: `numpy.arcsinh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.81× | 0.77× | 0.83× |
| MathAtan | [TA-Lib: `ATAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.92× | 2.04× |
| MathAtanh | [NumPy: `numpy.arctanh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 1.02× | 0.81× | 0.88× |
| MathCbrt | [NumPy: `numpy.cbrt`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.87× | 0.80× | 0.82× |
| MathCeil | [TA-Lib: `CEIL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.58× | 1.90× | 1.94× |
| MathCos | [TA-Lib: `COS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 2.14× | 1.98× |
| MathCosh | [TA-Lib: `COSH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.22× | 1.99× | 2.03× |
| MathCot | [NumPy: `numpy.tan reciprocal`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.96× | 1.07× | 0.90× |
| MathDegrees | [NumPy: `numpy.degrees`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.83× | 0.90× | 0.89× |
| MathDivide | [TA-Lib: `DIV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 1.96× | 1.91× |
| MathExp | [TA-Lib: `EXP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.58× | 2.03× | 1.95× |
| MathFloor | [TA-Lib: `FLOOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.29× | 1.79× | 1.92× |
| MathLn | [TA-Lib: `LN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.57× | 1.97× | 1.91× |
| MathLog10 | [TA-Lib: `LOG10`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 1.81× | 1.82× |
| MathLog1p | [NumPy: `numpy.log1p`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.84× | 0.82× | 0.82× |
| MathMultiply | [TA-Lib: `MULT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 1.91× | 1.83× |
| MathRadians | [NumPy: `numpy.radians`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.88× | 0.87× | 0.84× |
| MathSin | [TA-Lib: `SIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.99× | 1.89× |
| MathSinh | [TA-Lib: `SINH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 1.91× | 1.90× |
| MathSqrt | [TA-Lib: `SQRT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.89× | 1.76× | 1.96× |
| MathSubtract | [TA-Lib: `SUB`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 1.86× | 1.97× |
| MathTan | [TA-Lib: `TAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.88× | 2.00× |
| MathTanh | [TA-Lib: `TANH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.92× | 1.84× |
| McGinleyDynamic | [pandas-ta-classic: `pandas-ta-classic.mcginley`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| MedianPrice | [TA-Lib: `MEDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.97× | 1.86× |
| MesaAdaptiveMovingAverage | [TA-Lib: `MAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.11× | 2.08× | 2.21× |
| MinusDirectionalIndicator | [TA-Lib: `MINUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.80× | 1.92× |
| MinusDirectionalMovement | [TA-Lib: `MINUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 2.01× | 2.13× |
| Momentum | [TA-Lib: `MOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 2.04× | 1.94× |
| MoneyFlowIndex | [TA-Lib: `MFI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.99× | 2.05× | 1.95× |
| MovingAverage | [TA-Lib: `MA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 2.10× | 2.02× |
| MovingAverageConvergenceDivergence | [TA-Lib: `MACD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.22× | 2.44× | 2.23× |
| MovingAverageConvergenceDivergenceExtended | [TA-Lib: `MACDEXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.19× | 2.37× | 2.24× |
| MovingAverageConvergenceDivergenceFixed | [TA-Lib: `MACDFIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.10× | 1.99× | 2.25× |
| NegativeVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.negative_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| NormalizedAverageTrueRange | [TA-Lib: `NATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.90× | 1.84× |
| OnBalanceVolume | [TA-Lib: `OBV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.89× | 2.01× |
| OpeningRange | [pandas: `pandas.opening_range`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| OrderBlock | [smartmoneyconcepts: `smartmoneyconcepts.order_block`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| OrnsteinUhlenbeckHalfLife | [pandas: `pandas.ornstein_uhlenbeck_half_life`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| OutsideBar | [pandas: `pandas.outside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ParabolicMovingAverageStop | [pandas-ta-classic: `pandas-ta-classic.pmax`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ParabolicSar | [TA-Lib: `SAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.99× | 2.33× | 2.19× |
| ParabolicSarExtended | [TA-Lib: `SAREXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.52× | 2.67× | 2.57× |
| Parkinson | [pandas: `pandas.parkinson`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PercentagePriceOscillator | [TA-Lib: `PPO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.00× | 2.05× | 2.04× |
| PivotPoints | [pandas: `pandas.pivot_points`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PlusDirectionalIndicator | [TA-Lib: `PLUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.83× | 1.98× |
| PlusDirectionalMovement | [TA-Lib: `PLUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 2.09× | 1.96× |
| PositiveVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.positive_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| PremiumDiscount | [pandas: `pandas.premium_discount`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PreviousHighLow | [smartmoneyconcepts: `smartmoneyconcepts.previous_high_low`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| RateOfChange | [TA-Lib: `ROC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 2.03× | 1.89× |
| RateOfChangePercent | [TA-Lib: `ROCP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.99× | 1.87× |
| RateOfChangeRatio | [TA-Lib: `ROCR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 2.03× | 2.01× |
| RateOfChangeRatioPercent | [TA-Lib: `ROCR100`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 2.02× | 1.91× |
| RelativeMomentumIndex | [pandas: `pandas.rmi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RelativeStrengthIndex | [TA-Lib: `RSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.97× | 1.97× | 2.13× |
| Retracements | [smartmoneyconcepts: `smartmoneyconcepts.retracements`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| Rising | [pandas: `pandas.rising`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RogersSatchell | [pandas: `pandas.rogers_satchell`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollSpread | [pandas: `pandas.roll_spread`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingAlpha | [pandas: `pandas.rolling_alpha`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingArgmax | [TA-Lib: `MAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 2.00× | 2.07× |
| RollingArgmin | [TA-Lib: `MININDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.89× | 1.97× | 1.80× |
| RollingAutocorr | [pandas: `pandas.rolling_autocorr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingAverageDeviation | [TA-Lib: `AVGDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 2.08× | 2.04× |
| RollingBeta | [TA-Lib: `BETA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.91× | 2.01× |
| RollingCalmar | [pandas: `pandas.rolling_calmar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingCorrelation | [TA-Lib: `CORREL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.98× | 2.09× |
| RollingCov | [pandas: `pandas.rolling_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingEntropy | [pandas: `pandas.rolling_entropy`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingInformationRatio | [pandas: `pandas.rolling_information_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingKurtosis | [pandas: `pandas.rolling_kurtosis`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingLinearRegression | [TA-Lib: `LINEARREG`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.98× | 2.00× | 1.92× |
| RollingLinearRegressionAngle | [TA-Lib: `LINEARREG_ANGLE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.88× | 1.93× |
| RollingLinearRegressionIntercept | [TA-Lib: `LINEARREG_INTERCEPT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.95× | 1.94× | 2.10× |
| RollingLinearRegressionSlope | [TA-Lib: `LINEARREG_SLOPE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 2.03× | 1.89× |
| RollingMax | [TA-Lib: `MAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.89× | 1.94× | 2.08× |
| RollingMedian | [pandas: `pandas.rolling_median`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingMidpoint | [TA-Lib: `MIDPOINT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.88× | 2.10× |
| RollingMidprice | [TA-Lib: `MIDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.96× | 1.81× |
| RollingMin | [TA-Lib: `MIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.97× | 2.08× |
| RollingMinMax | [TA-Lib: `MINMAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.02× | 2.16× | 1.96× |
| RollingMinMaxIndex | [TA-Lib: `MINMAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.99× | 2.04× | 2.23× |
| RollingMode | [pandas: `pandas.rolling_mode`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingQuantile | [pandas: `pandas.rolling_quantile`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingRank | [pandas: `pandas.rolling_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSharpe | [pandas: `pandas.rolling_sharpe`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSkew | [pandas: `pandas.rolling_skew`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSortino | [pandas: `pandas.rolling_sortino`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingStandardDeviation | [TA-Lib: `STDDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.96× | 1.95× | 1.96× |
| RollingSum | [TA-Lib: `SUM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 2.01× | 1.96× |
| RollingTimeSeriesForecast | [TA-Lib: `TSF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 2.17× | 2.00× |
| RollingVariance | [TA-Lib: `VAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 2.04× | 1.98× |
| RollingVolumeWeightedAveragePrice | [pandas: `pandas.rolling_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingWinsorize | [pandas: `pandas.rolling_winsorize`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingZScore | [pandas: `pandas.rolling_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SchaffTrendCycle | [pandas-ta-classic: `pandas_ta_classic.stc`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SessionVolumeLevels | [pandas: `pandas.session_volume_levels`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Sessions | [smartmoneyconcepts: `smartmoneyconcepts.sessions`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| SignalDelay | [pandas: `pandas.signal_delay`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SignedPower | [NumPy: `numpy.sign/numpy.abs/numpy.power`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.96× | 0.89× | 0.88× |
| SimpleMovingAverage | [TA-Lib: `SMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.93× | 1.98× | 1.84× |
| SmoothedTrendChannel | [pandas: `pandas.ssl_channel`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SpreadZScore | [pandas: `pandas.spread_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Squeeze | [pandas-ta-classic: `pandas-ta-classic.squeeze`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SqueezePro | [pandas-ta-classic: `pandas-ta-classic.squeeze_pro`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| StochasticOscillator | [TA-Lib: `STOCH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.95× | 2.16× | 1.98× |
| StochasticRelativeStrengthIndex | [TA-Lib: `STOCHRSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.29× | 2.16× | 2.22× |
| Supertrend | [pandas-ta-classic: `pandas-ta-classic.supertrend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SwingHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.swing_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| TimeSeriesRank | [pandas: `pandas.time_series_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| TomDeMarkSequential | [pandas-ta-classic: `pandas-ta-classic.td_sequential`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| TriangularMovingAverage | [TA-Lib: `TRIMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 2.07× | 2.09× |
| TripleExponentialAverage | [TA-Lib: `T3`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 2.09× | 2.08× |
| TripleExponentialMovingAverage | [TA-Lib: `TEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 1.97× | 1.97× |
| TripleExponentialRateOfChange | [TA-Lib: `TRIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 1.91× | 1.87× |
| TrueRange | [TA-Lib: `TRANGE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.76× | 1.77× |
| TrueStrengthIndex | [pandas-ta-classic: `pandas-ta-classic.true_strength_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| TypicalPrice | [TA-Lib: `TYPPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.93× | 1.93× |
| UlcerIndex | [pandas-ta-classic: `pandas-ta-classic.ulcer_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| UltimateOscillator | [TA-Lib: `ULTOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 2.00× | 2.15× |
| ValueWhen | [pandas: `pandas.value_when`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| VariableIndexDynamicAverage | [pandas-ta-classic: `pandas-ta-classic.vidya`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| VariablePeriodMovingAverage | [TA-Lib: `MAVP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 2.02× | 2.02× |
| VolumePriceTrend | [pandas-ta-classic: `pandas-ta-classic.volume_price_trend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| VolumeWeightedMovingAverage | [pandas-ta-classic: `pandas_ta_classic.vwma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Vortex | [pandas-ta-classic: `pandas-ta-classic.vortex`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| WeightedClose | [TA-Lib: `WCLPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.87× | 1.93× |
| WeightedMovingAverage | [TA-Lib: `WMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.87× | 2.00× |
| WilliamsPercentR | [TA-Lib: `WILLR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 1.96× | 1.85× |
| YangZhang | [pandas: `pandas.yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ZeroLagExponentialMovingAverage | [pandas-ta-classic: `pandas_ta_classic.zlma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |

### 100 bars

| Class | Reference | 1 thread | 5 threads | 10 threads |
|---|---|---:|---:|---:|
| AbsolutePriceOscillator | [TA-Lib: `APO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.07× | 2.01× | 1.85× |
| AccelerationBands | [TA-Lib: `ACCBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 2.20× | 2.18× |
| AccumulationDistribution | [TA-Lib: `AD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.80× | 1.76× |
| AccumulationDistributionOscillator | [TA-Lib: `ADOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.98× | 1.89× |
| Amihud | [pandas: `pandas.amihud`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| AnchoredVolumeWeightedAveragePrice | [pandas: `pandas.anchored_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ArnaudLegouxMovingAverage | [pandas-ta-classic: `pandas-ta-classic.arnaud_legoux_moving_average`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Aroon | [TA-Lib: `AROON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.85× | 1.99× |
| AroonOscillator | [TA-Lib: `AROONOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.80× | 1.95× |
| AverageDailyDollarValue | [pandas: `pandas.average_daily_dollar_value`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| AverageDirectionalIndex | [TA-Lib: `ADX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.16× | 2.02× | 2.00× |
| AverageDirectionalIndexRating | [TA-Lib: `ADXR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.90× | 1.97× |
| AveragePrice | [TA-Lib: `AVGPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 1.68× | 1.81× |
| AverageTrueRange | [TA-Lib: `ATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.99× | 1.95× |
| AwesomeOscillator | [pandas-ta-classic: `pandas_ta_classic.ao`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| BalanceOfPower | [TA-Lib: `BOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.83× | 1.87× |
| BarsSince | [pandas: `pandas.bars_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| BollingerBands | [TA-Lib: `BBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.41× | 2.42× | 2.44× |
| BreakOfStructureChangeOfCharacter | [smartmoneyconcepts: `smartmoneyconcepts.break_of_structure_change_of_character`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| CandleAbandonedBaby | [TA-Lib: `CDLABANDONEDBABY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.80× | 1.95× |
| CandleAdvanceBlock | [TA-Lib: `CDLADVANCEBLOCK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.73× | 1.90× |
| CandleBeltHold | [TA-Lib: `CDLBELTHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.74× | 1.73× |
| CandleBreakaway | [TA-Lib: `CDLBREAKAWAY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.65× | 1.88× | 1.84× |
| CandleClosingMarubozu | [TA-Lib: `CDLCLOSINGMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.71× | 1.82× |
| CandleConcealBabySwall | [TA-Lib: `CDLCONCEALBABYSWALL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.76× | 1.82× |
| CandleCounterAttack | [TA-Lib: `CDLCOUNTERATTACK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.71× | 1.76× |
| CandleDarkCloudCover | [TA-Lib: `CDLDARKCLOUDCOVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.90× | 1.85× |
| CandleDoji | [TA-Lib: `CDLDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.73× | 1.67× |
| CandleDojiStar | [TA-Lib: `CDLDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.75× | 1.75× |
| CandleDragonflyDoji | [TA-Lib: `CDLDRAGONFLYDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.64× | 1.76× |
| CandleEngulfing | [TA-Lib: `CDLENGULFING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.68× | 1.84× |
| CandleEveningDojiStar | [TA-Lib: `CDLEVENINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.85× | 1.85× |
| CandleEveningStar | [TA-Lib: `CDLEVENINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.77× | 1.79× |
| CandleGapSideSideWhite | [TA-Lib: `CDLGAPSIDESIDEWHITE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.78× | 1.73× |
| CandleGravestoneDoji | [TA-Lib: `CDLGRAVESTONEDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.52× | 1.76× | 1.86× |
| CandleHammer | [TA-Lib: `CDLHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.75× | 1.86× |
| CandleHangingMan | [TA-Lib: `CDLHANGINGMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.76× | 1.76× |
| CandleHarami | [TA-Lib: `CDLHARAMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.74× | 1.75× |
| CandleHaramiCross | [TA-Lib: `CDLHARAMICROSS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 1.67× | 1.74× |
| CandleHighWave | [TA-Lib: `CDLHIGHWAVE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.66× | 1.71× |
| CandleHikkake | [TA-Lib: `CDLHIKKAKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.79× | 1.78× |
| CandleHikkakeModified | [TA-Lib: `CDLHIKKAKEMOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.89× | 1.86× |
| CandleHomingPigeon | [TA-Lib: `CDLHOMINGPIGEON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.71× | 1.78× |
| CandleIdenticalThreeCrows | [TA-Lib: `CDLIDENTICAL3CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.80× | 1.79× |
| CandleInNeck | [TA-Lib: `CDLINNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.72× | 1.76× |
| CandleInvertedHammer | [TA-Lib: `CDLINVERTEDHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.80× | 1.50× |
| CandleKicking | [TA-Lib: `CDLKICKING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.77× | 1.74× |
| CandleKickingByLength | [TA-Lib: `CDLKICKINGBYLENGTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.84× | 1.79× |
| CandleLadderBottom | [TA-Lib: `CDLLADDERBOTTOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.67× | 1.83× |
| CandleLongLeggedDoji | [TA-Lib: `CDLLONGLEGGEDDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.79× | 1.82× |
| CandleLongLine | [TA-Lib: `CDLLONGLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.67× | 1.83× |
| CandleMarubozu | [TA-Lib: `CDLMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 1.80× | 1.71× |
| CandleMatHold | [TA-Lib: `CDLMATHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.92× | 1.86× |
| CandleMatchingLow | [TA-Lib: `CDLMATCHINGLOW`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 1.79× | 1.81× |
| CandleMorningDojiStar | [TA-Lib: `CDLMORNINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 1.97× | 1.79× |
| CandleMorningStar | [TA-Lib: `CDLMORNINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 1.94× | 1.89× |
| CandleOnNeck | [TA-Lib: `CDLONNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 1.77× | 1.73× |
| CandlePiercing | [TA-Lib: `CDLPIERCING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.75× | 1.59× |
| CandleRickshawman | [TA-Lib: `CDLRICKSHAWMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.66× | 1.79× |
| CandleRiseFallThreeMethods | [TA-Lib: `CDLRISEFALL3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.73× | 1.74× |
| CandleSeparatingLines | [TA-Lib: `CDLSEPARATINGLINES`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.77× | 1.81× |
| CandleShootingStar | [TA-Lib: `CDLSHOOTINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.61× | 1.83× | 1.74× |
| CandleShortLine | [TA-Lib: `CDLSHORTLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 1.81× | 1.65× |
| CandleSpinningTop | [TA-Lib: `CDLSPINNINGTOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.76× | 1.74× |
| CandleStalledPattern | [TA-Lib: `CDLSTALLEDPATTERN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.61× | 1.79× | 1.78× |
| CandleStickSandwich | [TA-Lib: `CDLSTICKSANDWICH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.78× | 1.77× |
| CandleTakuri | [TA-Lib: `CDLTAKURI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.75× | 1.77× |
| CandleTasukiGap | [TA-Lib: `CDLTASUKIGAP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 1.70× | 1.78× |
| CandleThreeBlackCrows | [TA-Lib: `CDL3BLACKCROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.81× | 1.80× |
| CandleThreeInside | [TA-Lib: `CDL3INSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 1.81× | 1.78× |
| CandleThreeLineStrike | [TA-Lib: `CDL3LINESTRIKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.54× | 1.64× | 1.69× |
| CandleThreeOutside | [TA-Lib: `CDL3OUTSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.75× | 1.72× |
| CandleThreeStarsInSouth | [TA-Lib: `CDL3STARSINSOUTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.56× | 1.84× | 1.72× |
| CandleThreeWhiteSoldiers | [TA-Lib: `CDL3WHITESOLDIERS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.59× | 1.76× | 1.74× |
| CandleThrusting | [TA-Lib: `CDLTHRUSTING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.73× | 1.87× |
| CandleTriStar | [TA-Lib: `CDLTRISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.67× | 1.88× |
| CandleTwoCrows | [TA-Lib: `CDL2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.65× | 1.74× | 1.76× |
| CandleUniqueThreeRiver | [TA-Lib: `CDLUNIQUE3RIVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.53× | 1.68× | 1.78× |
| CandleUpDownSideGapThreeMethods | [TA-Lib: `CDLXSIDEGAP3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.82× | 1.82× |
| CandleUpsideGapTwoCrows | [TA-Lib: `CDLUPSIDEGAP2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.71× | 1.72× |
| ChaikinMoneyFlow | [pandas-ta-classic: `pandas_ta_classic.cmf`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ChaikinVolatility | [pandas-ta-classic: `pandas-ta-classic.chaikin_volatility`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ChandeMomentumOscillator | [TA-Lib: `CMO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 2.11× | 2.00× |
| CloseToCloseSigma | [pandas: `pandas.close_to_close_sigma`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| CommodityChannelIndex | [TA-Lib: `CCI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.89× | 1.89× |
| Crossover | [pandas-ta-classic: `pandas-ta-classic.crossover`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Crossunder | [pandas-ta-classic: `pandas-ta-classic.crossunder`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| CumulativeCount | [pandas: `pandas.cumulative_count`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| CumulativeMaximum | [Polars: `Polars.cumulative_maximum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.21× | 2.56× | 2.71× |
| CumulativeMinimum | [Polars: `Polars.cumulative_minimum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 2.59× | 2.63× | 2.54× |
| CumulativeProduct | [Polars: `Polars.cumulative_product`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.17× | 2.55× | 2.49× |
| CumulativeSum | [Polars: `Polars.cumulative_sum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.18× | 2.65× | 2.59× |
| CumulativeSumControlChart | [pandas: `pandas.cumulative_sum_control_chart`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| DecayLinear | [pandas: `pandas.decay_linear`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| DetrendedPriceOscillator | [pandas-ta-classic: `pandas_ta_classic.dpo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| DirectionalMovementIndex | [TA-Lib: `DX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 1.00× | 2.03× |
| DonchianChannels | [pandas-ta-classic: `pandas_ta_classic.donchian`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| DoubleExponentialMovingAverage | [TA-Lib: `DEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 1.91× | 2.03× |
| Drawdown | [pandas: `pandas.drawdown`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| EaseOfMovement | [pandas-ta-classic: `pandas-ta-classic.ease_of_movement`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| EqualHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.equal_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| EvenBetterSinewave | [pandas-ta-classic: `pandas_ta_classic.ebsw`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ExponentialMovingAverage | [TA-Lib: `EMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.30× | 1.97× | 2.09× |
| ExponentiallyWeightedCorrelation | [pandas: `pandas.ewm_corr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedCovariance | [pandas: `pandas.ewm_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedStandardDeviation | [pandas: `pandas.ewm_std`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedSum | [pandas: `pandas.exponentially_weighted_sum`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedVariance | [Polars: `Polars.ewm_var`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.08× | 2.73× | 2.79× |
| FairValueGap | [smartmoneyconcepts: `smartmoneyconcepts.fair_value_gap`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| Falling | [pandas: `pandas.falling`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FastStochasticOscillator | [TA-Lib: `STOCHF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.99× | 1.65× | 1.99× |
| FibonacciRetracement | [pandas: `pandas.fib_retracement`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FisherTransform | [pandas-ta-classic: `pandas_ta_classic.fisher`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ForceIndex | [pandas-ta-classic: `pandas_ta_classic.efi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| FracDiff | [pandas: `pandas.frac_diff`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FractalDimension | [pandas: `pandas.fractal_dimension`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GapDown | [pandas: `pandas.gap_down`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GapUp | [pandas: `pandas.gap_up`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GarmanKlass | [pandas: `pandas.garman_klass`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GarmanKlassYangZhang | [pandas: `pandas.garman_klass_yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HedgeRatio | [pandas: `pandas.hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HeikinAshi | [pandas-ta-classic: `pandas-ta-classic.heikin_ashi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| HigherHigh | [pandas: `pandas.higher_high`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HighestSince | [pandas: `pandas.highest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HilbertTransformDominantCyclePeriod | [TA-Lib: `HT_DCPERIOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.92× | 1.76× |
| HilbertTransformDominantCyclePhase | [TA-Lib: `HT_DCPHASE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.10× | 2.46× | 2.41× |
| HilbertTransformPhasor | [TA-Lib: `HT_PHASOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.89× | 2.06× |
| HilbertTransformSineWave | [TA-Lib: `HT_SINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 2.41× | 2.49× |
| HilbertTransformTrendMode | [TA-Lib: `HT_TRENDMODE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 2.62× | 2.40× |
| HilbertTransformTrendline | [TA-Lib: `HT_TRENDLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.87× | 2.00× |
| HullMovingAverage | [pandas-ta-classic: `pandas_ta_classic.hma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Hurst | [pandas: `pandas.hurst`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Ichimoku | [pandas-ta-classic: `pandas-ta-classic.ichimoku`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| InsideBar | [pandas: `pandas.inside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| IntradayMomentumIndex | [TA-Lib: `IMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.98× | 2.05× | 1.93× |
| JurikMovingAverage | [pandas-ta-classic: `pandas-ta-classic.jma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KalmanHedgeRatio | [pandas: `pandas.kalman_hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| KaufmanAdaptiveMovingAverage | [TA-Lib: `KAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.01× | 1.84× | 1.97× |
| KeltnerChannels | [pandas-ta-classic: `pandas-ta-classic.keltner_channels`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KlingerVolumeOscillator | [pandas-ta-classic: `pandas-ta-classic.kvo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KnowSureThing | [pandas-ta-classic: `pandas-ta-classic.know_sure_thing`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Lag | [pandas: `pandas.lag`](https://pandas.pydata.org/docs/reference/window.html) | 2.68× | 2.91× | 2.85× |
| LaguerreRelativeStrengthIndex | [pandas: `pandas.laguerre_rsi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Liquidity | [smartmoneyconcepts: `smartmoneyconcepts.liquidity`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| LogReturn | [pandas-ta-classic: `pandas_ta_classic.log_return`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | 4.51× | 5.32× | 5.10× |
| LowerLow | [pandas: `pandas.lower_low`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| LowestSince | [pandas: `pandas.lowest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| MassIndex | [pandas-ta-classic: `pandas-ta-classic.mass_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| MathAbs | [NumPy: `numpy.abs`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.84× | 0.85× | 0.89× |
| MathAcos | [TA-Lib: `ACOS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.83× | 1.79× |
| MathAcosh | [NumPy: `numpy.arccosh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.87× | 0.86× | 0.86× |
| MathAdd | [TA-Lib: `ADD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 1.93× | 1.89× |
| MathAsin | [TA-Lib: `ASIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.82× | 1.80× |
| MathAsinh | [NumPy: `numpy.arcsinh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.92× | 0.88× | 0.88× |
| MathAtan | [TA-Lib: `ATAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.89× | 1.92× |
| MathAtanh | [NumPy: `numpy.arctanh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.90× | 0.82× | 0.86× |
| MathCbrt | [NumPy: `numpy.cbrt`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.90× | 0.98× | 0.83× |
| MathCeil | [TA-Lib: `CEIL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.96× | 2.02× | 1.93× |
| MathCos | [TA-Lib: `COS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.98× | 1.90× |
| MathCosh | [TA-Lib: `COSH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.80× | 1.93× |
| MathCot | [NumPy: `numpy.tan reciprocal`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.92× | 0.85× | 0.88× |
| MathDegrees | [NumPy: `numpy.degrees`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.89× | 0.85× | 0.84× |
| MathDivide | [TA-Lib: `DIV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.85× | 1.96× |
| MathExp | [TA-Lib: `EXP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 1.72× | 1.87× |
| MathFloor | [TA-Lib: `FLOOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.98× | 1.93× | 1.97× |
| MathLn | [TA-Lib: `LN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.78× | 1.79× |
| MathLog10 | [TA-Lib: `LOG10`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.63× | 1.84× |
| MathLog1p | [NumPy: `numpy.log1p`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.94× | 0.83× | 0.80× |
| MathMultiply | [TA-Lib: `MULT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 1.84× | 1.88× |
| MathRadians | [NumPy: `numpy.radians`](https://numpy.org/doc/stable/reference/ufuncs.html) | 1.24× | 0.80× | 0.83× |
| MathSin | [TA-Lib: `SIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.84× | 1.88× |
| MathSinh | [TA-Lib: `SINH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.88× | 1.85× |
| MathSqrt | [TA-Lib: `SQRT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 1.79× | 1.91× |
| MathSubtract | [TA-Lib: `SUB`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.90× | 1.98× | 1.86× |
| MathTan | [TA-Lib: `TAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.87× | 1.81× |
| MathTanh | [TA-Lib: `TANH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.69× | 1.92× |
| McGinleyDynamic | [pandas-ta-classic: `pandas-ta-classic.mcginley`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| MedianPrice | [TA-Lib: `MEDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.89× | 1.86× |
| MesaAdaptiveMovingAverage | [TA-Lib: `MAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.98× | 2.07× |
| MinusDirectionalIndicator | [TA-Lib: `MINUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.91× | 1.87× | 2.07× |
| MinusDirectionalMovement | [TA-Lib: `MINUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 2.00× | 2.15× |
| Momentum | [TA-Lib: `MOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.89× | 1.93× |
| MoneyFlowIndex | [TA-Lib: `MFI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 1.87× | 1.90× |
| MovingAverage | [TA-Lib: `MA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.91× | 2.33× | 2.11× |
| MovingAverageConvergenceDivergence | [TA-Lib: `MACD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.19× | 2.20× | 2.25× |
| MovingAverageConvergenceDivergenceExtended | [TA-Lib: `MACDEXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.09× | 2.28× | 2.26× |
| MovingAverageConvergenceDivergenceFixed | [TA-Lib: `MACDFIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.12× | 2.28× | 2.25× |
| NegativeVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.negative_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| NormalizedAverageTrueRange | [TA-Lib: `NATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 1.90× | 1.95× |
| OnBalanceVolume | [TA-Lib: `OBV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 1.89× | 1.89× |
| OpeningRange | [pandas: `pandas.opening_range`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| OrderBlock | [smartmoneyconcepts: `smartmoneyconcepts.order_block`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| OrnsteinUhlenbeckHalfLife | [pandas: `pandas.ornstein_uhlenbeck_half_life`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| OutsideBar | [pandas: `pandas.outside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ParabolicMovingAverageStop | [pandas-ta-classic: `pandas-ta-classic.pmax`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ParabolicSar | [TA-Lib: `SAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 1.99× | 2.14× |
| ParabolicSarExtended | [TA-Lib: `SAREXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.13× | 2.27× | 2.32× |
| Parkinson | [pandas: `pandas.parkinson`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PercentagePriceOscillator | [TA-Lib: `PPO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.60× | 1.99× | 1.93× |
| PivotPoints | [pandas: `pandas.pivot_points`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PlusDirectionalIndicator | [TA-Lib: `PLUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 2.01× | 1.83× |
| PlusDirectionalMovement | [TA-Lib: `PLUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 2.06× | 1.93× |
| PositiveVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.positive_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| PremiumDiscount | [pandas: `pandas.premium_discount`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PreviousHighLow | [smartmoneyconcepts: `smartmoneyconcepts.previous_high_low`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| RateOfChange | [TA-Lib: `ROC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 1.82× | 1.91× |
| RateOfChangePercent | [TA-Lib: `ROCP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 1.95× | 1.93× |
| RateOfChangeRatio | [TA-Lib: `ROCR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.85× | 1.89× |
| RateOfChangeRatioPercent | [TA-Lib: `ROCR100`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.14× | 1.86× | 2.04× |
| RelativeMomentumIndex | [pandas: `pandas.rmi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RelativeStrengthIndex | [TA-Lib: `RSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.93× | 1.71× | 1.95× |
| Retracements | [smartmoneyconcepts: `smartmoneyconcepts.retracements`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| Rising | [pandas: `pandas.rising`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RogersSatchell | [pandas: `pandas.rogers_satchell`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollSpread | [pandas: `pandas.roll_spread`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingAlpha | [pandas: `pandas.rolling_alpha`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingArgmax | [TA-Lib: `MAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 2.01× | 1.97× |
| RollingArgmin | [TA-Lib: `MININDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 1.96× | 1.84× |
| RollingAutocorr | [pandas: `pandas.rolling_autocorr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingAverageDeviation | [TA-Lib: `AVGDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.95× | 1.96× |
| RollingBeta | [TA-Lib: `BETA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.93× | 2.02× |
| RollingCalmar | [pandas: `pandas.rolling_calmar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingCorrelation | [TA-Lib: `CORREL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 2.23× | 1.98× |
| RollingCov | [pandas: `pandas.rolling_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingEntropy | [pandas: `pandas.rolling_entropy`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingInformationRatio | [pandas: `pandas.rolling_information_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingKurtosis | [pandas: `pandas.rolling_kurtosis`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingLinearRegression | [TA-Lib: `LINEARREG`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 1.87× | 1.99× |
| RollingLinearRegressionAngle | [TA-Lib: `LINEARREG_ANGLE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 2.04× | 2.06× |
| RollingLinearRegressionIntercept | [TA-Lib: `LINEARREG_INTERCEPT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.91× | 2.06× |
| RollingLinearRegressionSlope | [TA-Lib: `LINEARREG_SLOPE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.91× | 2.00× | 2.05× |
| RollingMax | [TA-Lib: `MAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.58× | 2.03× | 1.95× |
| RollingMedian | [pandas: `pandas.rolling_median`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingMidpoint | [TA-Lib: `MIDPOINT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 1.88× | 1.98× |
| RollingMidprice | [TA-Lib: `MIDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 1.89× | 1.87× |
| RollingMin | [TA-Lib: `MIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.57× | 1.82× | 2.15× |
| RollingMinMax | [TA-Lib: `MINMAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 1.98× | 2.14× |
| RollingMinMaxIndex | [TA-Lib: `MINMAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.60× | 2.12× | 1.98× |
| RollingMode | [pandas: `pandas.rolling_mode`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingQuantile | [pandas: `pandas.rolling_quantile`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingRank | [pandas: `pandas.rolling_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSharpe | [pandas: `pandas.rolling_sharpe`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSkew | [pandas: `pandas.rolling_skew`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSortino | [pandas: `pandas.rolling_sortino`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingStandardDeviation | [TA-Lib: `STDDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.98× | 1.97× | 1.79× |
| RollingSum | [TA-Lib: `SUM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.92× | 2.04× | 1.79× |
| RollingTimeSeriesForecast | [TA-Lib: `TSF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 2.06× | 1.94× |
| RollingVariance | [TA-Lib: `VAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.98× | 1.97× |
| RollingVolumeWeightedAveragePrice | [pandas: `pandas.rolling_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingWinsorize | [pandas: `pandas.rolling_winsorize`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingZScore | [pandas: `pandas.rolling_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SchaffTrendCycle | [pandas-ta-classic: `pandas_ta_classic.stc`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SessionVolumeLevels | [pandas: `pandas.session_volume_levels`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Sessions | [smartmoneyconcepts: `smartmoneyconcepts.sessions`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| SignalDelay | [pandas: `pandas.signal_delay`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SignedPower | [NumPy: `numpy.sign/numpy.abs/numpy.power`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.98× | 0.87× | 0.94× |
| SimpleMovingAverage | [TA-Lib: `SMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.94× | 1.96× | 2.06× |
| SmoothedTrendChannel | [pandas: `pandas.ssl_channel`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SpreadZScore | [pandas: `pandas.spread_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Squeeze | [pandas-ta-classic: `pandas-ta-classic.squeeze`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SqueezePro | [pandas-ta-classic: `pandas-ta-classic.squeeze_pro`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| StochasticOscillator | [TA-Lib: `STOCH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 2.06× | 1.93× |
| StochasticRelativeStrengthIndex | [TA-Lib: `STOCHRSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.96× | 2.20× | 2.16× |
| Supertrend | [pandas-ta-classic: `pandas-ta-classic.supertrend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SwingHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.swing_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| TimeSeriesRank | [pandas: `pandas.time_series_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| TomDeMarkSequential | [pandas-ta-classic: `pandas-ta-classic.td_sequential`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| TriangularMovingAverage | [TA-Lib: `TRIMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.86× | 1.61× |
| TripleExponentialAverage | [TA-Lib: `T3`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.95× | 1.98× | 2.00× |
| TripleExponentialMovingAverage | [TA-Lib: `TEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.07× | 2.04× | 1.98× |
| TripleExponentialRateOfChange | [TA-Lib: `TRIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.56× | 1.96× | 2.13× |
| TrueRange | [TA-Lib: `TRANGE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.85× | 1.90× |
| TrueStrengthIndex | [pandas-ta-classic: `pandas-ta-classic.true_strength_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| TypicalPrice | [TA-Lib: `TYPPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.88× | 1.78× |
| UlcerIndex | [pandas-ta-classic: `pandas-ta-classic.ulcer_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| UltimateOscillator | [TA-Lib: `ULTOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 2.00× | 1.98× |
| ValueWhen | [pandas: `pandas.value_when`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| VariableIndexDynamicAverage | [pandas-ta-classic: `pandas-ta-classic.vidya`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| VariablePeriodMovingAverage | [TA-Lib: `MAVP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 2.09× | 1.95× |
| VolumePriceTrend | [pandas-ta-classic: `pandas-ta-classic.volume_price_trend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| VolumeWeightedMovingAverage | [pandas-ta-classic: `pandas_ta_classic.vwma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Vortex | [pandas-ta-classic: `pandas-ta-classic.vortex`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| WeightedClose | [TA-Lib: `WCLPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.78× | 1.85× | 1.96× |
| WeightedMovingAverage | [TA-Lib: `WMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.90× | 2.07× | 2.03× |
| WilliamsPercentR | [TA-Lib: `WILLR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.94× | 1.78× |
| YangZhang | [pandas: `pandas.yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ZeroLagExponentialMovingAverage | [pandas-ta-classic: `pandas_ta_classic.zlma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |

### 1,000 bars

| Class | Reference | 1 thread | 5 threads | 10 threads |
|---|---|---:|---:|---:|
| AbsolutePriceOscillator | [TA-Lib: `APO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 2.31× | 1.88× |
| AccelerationBands | [TA-Lib: `ACCBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 2.04× | 2.18× |
| AccumulationDistribution | [TA-Lib: `AD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 1.80× | 1.67× |
| AccumulationDistributionOscillator | [TA-Lib: `ADOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.91× | 1.90× |
| Amihud | [pandas: `pandas.amihud`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| AnchoredVolumeWeightedAveragePrice | [pandas: `pandas.anchored_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ArnaudLegouxMovingAverage | [pandas-ta-classic: `pandas-ta-classic.arnaud_legoux_moving_average`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Aroon | [TA-Lib: `AROON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.58× | 2.02× | 2.26× |
| AroonOscillator | [TA-Lib: `AROONOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 2.12× | 2.05× |
| AverageDailyDollarValue | [pandas: `pandas.average_daily_dollar_value`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| AverageDirectionalIndex | [TA-Lib: `ADX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 2.04× | 1.98× |
| AverageDirectionalIndexRating | [TA-Lib: `ADXR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.69× | 1.91× | 2.06× |
| AveragePrice | [TA-Lib: `AVGPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.78× | 1.79× |
| AverageTrueRange | [TA-Lib: `ATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.93× | 1.98× |
| AwesomeOscillator | [pandas-ta-classic: `pandas_ta_classic.ao`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| BalanceOfPower | [TA-Lib: `BOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.85× | 1.78× | 1.76× |
| BarsSince | [pandas: `pandas.bars_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| BollingerBands | [TA-Lib: `BBANDS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.20× | 2.40× | 2.40× |
| BreakOfStructureChangeOfCharacter | [smartmoneyconcepts: `smartmoneyconcepts.break_of_structure_change_of_character`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| CandleAbandonedBaby | [TA-Lib: `CDLABANDONEDBABY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.58× | 1.86× | 2.05× |
| CandleAdvanceBlock | [TA-Lib: `CDLADVANCEBLOCK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 2.05× | 2.10× |
| CandleBeltHold | [TA-Lib: `CDLBELTHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.53× | 1.99× | 1.98× |
| CandleBreakaway | [TA-Lib: `CDLBREAKAWAY`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.56× | 1.88× | 1.89× |
| CandleClosingMarubozu | [TA-Lib: `CDLCLOSINGMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.53× | 1.93× | 2.01× |
| CandleConcealBabySwall | [TA-Lib: `CDLCONCEALBABYSWALL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.53× | 1.80× | 1.86× |
| CandleCounterAttack | [TA-Lib: `CDLCOUNTERATTACK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.93× | 1.97× |
| CandleDarkCloudCover | [TA-Lib: `CDLDARKCLOUDCOVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.97× | 1.97× |
| CandleDoji | [TA-Lib: `CDLDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.60× | 1.81× |
| CandleDojiStar | [TA-Lib: `CDLDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.58× | 1.84× | 1.91× |
| CandleDragonflyDoji | [TA-Lib: `CDLDRAGONFLYDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.46× | 1.99× | 1.81× |
| CandleEngulfing | [TA-Lib: `CDLENGULFING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 2.16× | 1.73× |
| CandleEveningDojiStar | [TA-Lib: `CDLEVENINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 1.99× | 2.01× |
| CandleEveningStar | [TA-Lib: `CDLEVENINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 2.08× | 1.93× |
| CandleGapSideSideWhite | [TA-Lib: `CDLGAPSIDESIDEWHITE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 2.15× | 2.14× |
| CandleGravestoneDoji | [TA-Lib: `CDLGRAVESTONEDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.61× | 1.59× | 1.95× |
| CandleHammer | [TA-Lib: `CDLHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.59× | 2.03× | 1.98× |
| CandleHangingMan | [TA-Lib: `CDLHANGINGMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.61× | 2.09× | 2.10× |
| CandleHarami | [TA-Lib: `CDLHARAMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.94× | 1.96× |
| CandleHaramiCross | [TA-Lib: `CDLHARAMICROSS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.61× | 1.95× | 1.95× |
| CandleHighWave | [TA-Lib: `CDLHIGHWAVE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.56× | 1.95× | 1.96× |
| CandleHikkake | [TA-Lib: `CDLHIKKAKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.52× | 1.79× | 1.61× |
| CandleHikkakeModified | [TA-Lib: `CDLHIKKAKEMOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 1.82× | 1.79× |
| CandleHomingPigeon | [TA-Lib: `CDLHOMINGPIGEON`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.57× | 1.86× | 1.81× |
| CandleIdenticalThreeCrows | [TA-Lib: `CDLIDENTICAL3CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 1.98× | 1.90× |
| CandleInNeck | [TA-Lib: `CDLINNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.60× | 1.92× | 1.82× |
| CandleInvertedHammer | [TA-Lib: `CDLINVERTEDHAMMER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.59× | 2.04× | 1.97× |
| CandleKicking | [TA-Lib: `CDLKICKING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 2.16× | 1.99× |
| CandleKickingByLength | [TA-Lib: `CDLKICKINGBYLENGTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 2.08× | 2.09× |
| CandleLadderBottom | [TA-Lib: `CDLLADDERBOTTOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.92× | 1.77× |
| CandleLongLeggedDoji | [TA-Lib: `CDLLONGLEGGEDDOJI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.87× | 1.90× |
| CandleLongLine | [TA-Lib: `CDLLONGLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.97× | 1.98× |
| CandleMarubozu | [TA-Lib: `CDLMARUBOZU`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.48× | 1.99× | 1.90× |
| CandleMatHold | [TA-Lib: `CDLMATHOLD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.61× | 2.11× | 2.06× |
| CandleMatchingLow | [TA-Lib: `CDLMATCHINGLOW`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.87× | 1.78× |
| CandleMorningDojiStar | [TA-Lib: `CDLMORNINGDOJISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 2.05× | 2.07× |
| CandleMorningStar | [TA-Lib: `CDLMORNINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.97× | 1.99× |
| CandleOnNeck | [TA-Lib: `CDLONNECK`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.51× | 1.97× | 1.83× |
| CandlePiercing | [TA-Lib: `CDLPIERCING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.94× | 1.89× |
| CandleRickshawman | [TA-Lib: `CDLRICKSHAWMAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.60× | 1.92× | 1.89× |
| CandleRiseFallThreeMethods | [TA-Lib: `CDLRISEFALL3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.47× | 1.97× | 1.92× |
| CandleSeparatingLines | [TA-Lib: `CDLSEPARATINGLINES`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 1.88× | 1.89× |
| CandleShootingStar | [TA-Lib: `CDLSHOOTINGSTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.59× | 2.02× | 1.98× |
| CandleShortLine | [TA-Lib: `CDLSHORTLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.58× | 2.12× | 2.10× |
| CandleSpinningTop | [TA-Lib: `CDLSPINNINGTOP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.60× | 2.04× | 1.89× |
| CandleStalledPattern | [TA-Lib: `CDLSTALLEDPATTERN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 1.97× | 1.96× |
| CandleStickSandwich | [TA-Lib: `CDLSTICKSANDWICH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.89× | 1.99× |
| CandleTakuri | [TA-Lib: `CDLTAKURI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.99× | 1.75× |
| CandleTasukiGap | [TA-Lib: `CDLTASUKIGAP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.97× | 2.10× |
| CandleThreeBlackCrows | [TA-Lib: `CDL3BLACKCROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.46× | 1.86× | 1.79× |
| CandleThreeInside | [TA-Lib: `CDL3INSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.52× | 1.92× | 1.98× |
| CandleThreeLineStrike | [TA-Lib: `CDL3LINESTRIKE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.47× | 1.83× | 1.84× |
| CandleThreeOutside | [TA-Lib: `CDL3OUTSIDE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.90× | 1.88× |
| CandleThreeStarsInSouth | [TA-Lib: `CDL3STARSINSOUTH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.80× | 1.95× |
| CandleThreeWhiteSoldiers | [TA-Lib: `CDL3WHITESOLDIERS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 1.93× | 2.02× |
| CandleThrusting | [TA-Lib: `CDLTHRUSTING`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.65× | 1.93× | 1.90× |
| CandleTriStar | [TA-Lib: `CDLTRISTAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.86× | 1.91× |
| CandleTwoCrows | [TA-Lib: `CDL2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.65× | 1.92× | 1.84× |
| CandleUniqueThreeRiver | [TA-Lib: `CDLUNIQUE3RIVER`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.58× | 1.80× | 1.90× |
| CandleUpDownSideGapThreeMethods | [TA-Lib: `CDLXSIDEGAP3METHODS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.87× | 1.81× |
| CandleUpsideGapTwoCrows | [TA-Lib: `CDLUPSIDEGAP2CROWS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.42× | 1.91× | 1.90× |
| ChaikinMoneyFlow | [pandas-ta-classic: `pandas_ta_classic.cmf`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ChaikinVolatility | [pandas-ta-classic: `pandas-ta-classic.chaikin_volatility`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ChandeMomentumOscillator | [TA-Lib: `CMO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 2.06× | 2.10× |
| CloseToCloseSigma | [pandas: `pandas.close_to_close_sigma`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| CommodityChannelIndex | [TA-Lib: `CCI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.60× | 2.23× | 2.17× |
| Crossover | [pandas-ta-classic: `pandas-ta-classic.crossover`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Crossunder | [pandas-ta-classic: `pandas-ta-classic.crossunder`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| CumulativeCount | [pandas: `pandas.cumulative_count`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| CumulativeMaximum | [Polars: `Polars.cumulative_maximum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.21× | 2.33× | 2.29× |
| CumulativeMinimum | [Polars: `Polars.cumulative_minimum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.03× | 2.39× | 2.32× |
| CumulativeProduct | [Polars: `Polars.cumulative_product`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.13× | 2.48× | 2.45× |
| CumulativeSum | [Polars: `Polars.cumulative_sum`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.09× | 2.44× | 2.46× |
| CumulativeSumControlChart | [pandas: `pandas.cumulative_sum_control_chart`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| DecayLinear | [pandas: `pandas.decay_linear`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| DetrendedPriceOscillator | [pandas-ta-classic: `pandas_ta_classic.dpo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| DirectionalMovementIndex | [TA-Lib: `DX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 1.17× | 1.93× |
| DonchianChannels | [pandas-ta-classic: `pandas_ta_classic.donchian`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| DoubleExponentialMovingAverage | [TA-Lib: `DEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.71× | 2.03× | 2.17× |
| Drawdown | [pandas: `pandas.drawdown`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| EaseOfMovement | [pandas-ta-classic: `pandas-ta-classic.ease_of_movement`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| EqualHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.equal_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| EvenBetterSinewave | [pandas-ta-classic: `pandas_ta_classic.ebsw`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ExponentialMovingAverage | [TA-Lib: `EMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 1.86× | 1.94× |
| ExponentiallyWeightedCorrelation | [pandas: `pandas.ewm_corr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedCovariance | [pandas: `pandas.ewm_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedStandardDeviation | [pandas: `pandas.ewm_std`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedSum | [pandas: `pandas.exponentially_weighted_sum`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ExponentiallyWeightedVariance | [Polars: `Polars.ewm_var`](https://docs.pola.rs/api/python/stable/reference/expressions/index.html) | 3.20× | 2.53× | 2.50× |
| FairValueGap | [smartmoneyconcepts: `smartmoneyconcepts.fair_value_gap`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| Falling | [pandas: `pandas.falling`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FastStochasticOscillator | [TA-Lib: `STOCHF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.79× | 2.16× | 2.08× |
| FibonacciRetracement | [pandas: `pandas.fib_retracement`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FisherTransform | [pandas-ta-classic: `pandas_ta_classic.fisher`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ForceIndex | [pandas-ta-classic: `pandas_ta_classic.efi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| FracDiff | [pandas: `pandas.frac_diff`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| FractalDimension | [pandas: `pandas.fractal_dimension`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GapDown | [pandas: `pandas.gap_down`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GapUp | [pandas: `pandas.gap_up`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GarmanKlass | [pandas: `pandas.garman_klass`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| GarmanKlassYangZhang | [pandas: `pandas.garman_klass_yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HedgeRatio | [pandas: `pandas.hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HeikinAshi | [pandas-ta-classic: `pandas-ta-classic.heikin_ashi`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| HigherHigh | [pandas: `pandas.higher_high`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HighestSince | [pandas: `pandas.highest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| HilbertTransformDominantCyclePeriod | [TA-Lib: `HT_DCPERIOD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.15× | 2.77× | 2.67× |
| HilbertTransformDominantCyclePhase | [TA-Lib: `HT_DCPHASE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.31× | 8.85× | 9.27× |
| HilbertTransformPhasor | [TA-Lib: `HT_PHASOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.46× | 2.90× | 2.67× |
| HilbertTransformSineWave | [TA-Lib: `HT_SINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 3.06× | 7.57× | 9.34× |
| HilbertTransformTrendMode | [TA-Lib: `HT_TRENDMODE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.48× | 7.56× | 8.77× |
| HilbertTransformTrendline | [TA-Lib: `HT_TRENDLINE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.25× | 2.87× | 3.07× |
| HullMovingAverage | [pandas-ta-classic: `pandas_ta_classic.hma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Hurst | [pandas: `pandas.hurst`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Ichimoku | [pandas-ta-classic: `pandas-ta-classic.ichimoku`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| InsideBar | [pandas: `pandas.inside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| IntradayMomentumIndex | [TA-Lib: `IMI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.67× | 3.23× | 3.22× |
| JurikMovingAverage | [pandas-ta-classic: `pandas-ta-classic.jma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KalmanHedgeRatio | [pandas: `pandas.kalman_hedge_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| KaufmanAdaptiveMovingAverage | [TA-Lib: `KAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 2.02× | 1.86× |
| KeltnerChannels | [pandas-ta-classic: `pandas-ta-classic.keltner_channels`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KlingerVolumeOscillator | [pandas-ta-classic: `pandas-ta-classic.kvo`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| KnowSureThing | [pandas-ta-classic: `pandas-ta-classic.know_sure_thing`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Lag | [pandas: `pandas.lag`](https://pandas.pydata.org/docs/reference/window.html) | 2.60× | 2.83× | 2.99× |
| LaguerreRelativeStrengthIndex | [pandas: `pandas.laguerre_rsi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Liquidity | [smartmoneyconcepts: `smartmoneyconcepts.liquidity`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| LogReturn | [pandas-ta-classic: `pandas_ta_classic.log_return`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | 4.47× | 5.48× | 5.42× |
| LowerLow | [pandas: `pandas.lower_low`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| LowestSince | [pandas: `pandas.lowest_since`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| MassIndex | [pandas-ta-classic: `pandas-ta-classic.mass_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| MathAbs | [NumPy: `numpy.abs`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.97× | 0.88× | 0.77× |
| MathAcos | [TA-Lib: `ACOS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 1.94× | 1.78× |
| MathAcosh | [NumPy: `numpy.arccosh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.92× | 0.91× | 0.91× |
| MathAdd | [TA-Lib: `ADD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.94× | 2.02× |
| MathAsin | [TA-Lib: `ASIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 2.16× | 2.01× |
| MathAsinh | [NumPy: `numpy.arcsinh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.93× | 0.92× | 0.96× |
| MathAtan | [TA-Lib: `ATAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.91× | 1.90× |
| MathAtanh | [NumPy: `numpy.arctanh`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.95× | 0.87× | 0.91× |
| MathCbrt | [NumPy: `numpy.cbrt`](https://numpy.org/doc/stable/reference/ufuncs.html) | 1.06× | 0.96× | 0.88× |
| MathCeil | [TA-Lib: `CEIL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 2.03× | 1.91× |
| MathCos | [TA-Lib: `COS`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 2.07× | 2.11× |
| MathCosh | [TA-Lib: `COSH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.06× | 2.02× | 2.01× |
| MathCot | [NumPy: `numpy.tan reciprocal`](https://numpy.org/doc/stable/reference/ufuncs.html) | 1.01× | 0.92× | 0.94× |
| MathDegrees | [NumPy: `numpy.degrees`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.84× | 0.89× | 0.95× |
| MathDivide | [TA-Lib: `DIV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 1.69× | 1.78× |
| MathExp | [TA-Lib: `EXP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 1.82× | 1.85× |
| MathFloor | [TA-Lib: `FLOOR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.89× | 1.74× | 1.89× |
| MathLn | [TA-Lib: `LN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 2.01× | 1.87× |
| MathLog10 | [TA-Lib: `LOG10`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.54× | 1.99× | 2.02× |
| MathLog1p | [NumPy: `numpy.log1p`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.91× | 0.93× | 0.87× |
| MathMultiply | [TA-Lib: `MULT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.08× | 1.88× | 1.94× |
| MathRadians | [NumPy: `numpy.radians`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.90× | 0.85× | 0.89× |
| MathSin | [TA-Lib: `SIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 2.09× | 2.13× |
| MathSinh | [TA-Lib: `SINH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 1.88× | 1.96× |
| MathSqrt | [TA-Lib: `SQRT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 1.87× | 2.00× |
| MathSubtract | [TA-Lib: `SUB`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.96× | 1.91× |
| MathTan | [TA-Lib: `TAN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.49× | 2.17× | 2.07× |
| MathTanh | [TA-Lib: `TANH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.83× | 1.89× |
| McGinleyDynamic | [pandas-ta-classic: `pandas-ta-classic.mcginley`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| MedianPrice | [TA-Lib: `MEDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 1.83× | 1.81× |
| MesaAdaptiveMovingAverage | [TA-Lib: `MAMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.49× | 2.63× | 2.72× |
| MinusDirectionalIndicator | [TA-Lib: `MINUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.65× | 1.93× | 1.99× |
| MinusDirectionalMovement | [TA-Lib: `MINUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.96× | 2.11× |
| Momentum | [TA-Lib: `MOM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.98× | 1.86× |
| MoneyFlowIndex | [TA-Lib: `MFI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 1.99× | 1.99× |
| MovingAverage | [TA-Lib: `MA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.89× | 1.95× | 1.96× |
| MovingAverageConvergenceDivergence | [TA-Lib: `MACD`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.07× | 2.47× | 2.37× |
| MovingAverageConvergenceDivergenceExtended | [TA-Lib: `MACDEXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.99× | 2.42× | 2.33× |
| MovingAverageConvergenceDivergenceFixed | [TA-Lib: `MACDFIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.86× | 2.50× | 2.41× |
| NegativeVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.negative_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| NormalizedAverageTrueRange | [TA-Lib: `NATR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.63× | 1.95× | 2.04× |
| OnBalanceVolume | [TA-Lib: `OBV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 1.95× | 1.89× |
| OpeningRange | [pandas: `pandas.opening_range`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| OrderBlock | [smartmoneyconcepts: `smartmoneyconcepts.order_block`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| OrnsteinUhlenbeckHalfLife | [pandas: `pandas.ornstein_uhlenbeck_half_life`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| OutsideBar | [pandas: `pandas.outside_bar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ParabolicMovingAverageStop | [pandas-ta-classic: `pandas-ta-classic.pmax`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| ParabolicSar | [TA-Lib: `SAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.99× | 2.12× | 2.13× |
| ParabolicSarExtended | [TA-Lib: `SAREXT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.93× | 2.37× | 2.42× |
| Parkinson | [pandas: `pandas.parkinson`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PercentagePriceOscillator | [TA-Lib: `PPO`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 2.04× | 1.82× |
| PivotPoints | [pandas: `pandas.pivot_points`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PlusDirectionalIndicator | [TA-Lib: `PLUS_DI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.82× | 2.02× | 1.99× |
| PlusDirectionalMovement | [TA-Lib: `PLUS_DM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 2.02× | 2.07× |
| PositiveVolumeIndex | [pandas-ta-classic: `pandas-ta-classic.positive_volume_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| PremiumDiscount | [pandas: `pandas.premium_discount`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| PreviousHighLow | [smartmoneyconcepts: `smartmoneyconcepts.previous_high_low`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| RateOfChange | [TA-Lib: `ROC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.81× | 1.89× | 1.87× |
| RateOfChangePercent | [TA-Lib: `ROCP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 1.85× | 1.90× |
| RateOfChangeRatio | [TA-Lib: `ROCR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.62× | 1.84× | 1.94× |
| RateOfChangeRatioPercent | [TA-Lib: `ROCR100`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 1.83× | 1.89× |
| RelativeMomentumIndex | [pandas: `pandas.rmi`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RelativeStrengthIndex | [TA-Lib: `RSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 2.08× | 2.10× |
| Retracements | [smartmoneyconcepts: `smartmoneyconcepts.retracements`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| Rising | [pandas: `pandas.rising`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RogersSatchell | [pandas: `pandas.rogers_satchell`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollSpread | [pandas: `pandas.roll_spread`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingAlpha | [pandas: `pandas.rolling_alpha`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingArgmax | [TA-Lib: `MAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 1.99× | 2.10× |
| RollingArgmin | [TA-Lib: `MININDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 2.11× | 1.76× |
| RollingAutocorr | [pandas: `pandas.rolling_autocorr`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingAverageDeviation | [TA-Lib: `AVGDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.05× | 2.10× | 2.27× |
| RollingBeta | [TA-Lib: `BETA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 2.03× | 2.02× |
| RollingCalmar | [pandas: `pandas.rolling_calmar`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingCorrelation | [TA-Lib: `CORREL`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.94× | 2.00× | 1.95× |
| RollingCov | [pandas: `pandas.rolling_cov`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingEntropy | [pandas: `pandas.rolling_entropy`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingInformationRatio | [pandas: `pandas.rolling_information_ratio`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingKurtosis | [pandas: `pandas.rolling_kurtosis`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingLinearRegression | [TA-Lib: `LINEARREG`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 2.07× | 2.22× |
| RollingLinearRegressionAngle | [TA-Lib: `LINEARREG_ANGLE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.66× | 2.50× | 2.12× |
| RollingLinearRegressionIntercept | [TA-Lib: `LINEARREG_INTERCEPT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.70× | 2.05× | 2.01× |
| RollingLinearRegressionSlope | [TA-Lib: `LINEARREG_SLOPE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 2.08× | 2.16× |
| RollingMax | [TA-Lib: `MAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.89× | 1.91× | 2.00× |
| RollingMedian | [pandas: `pandas.rolling_median`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingMidpoint | [TA-Lib: `MIDPOINT`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.75× | 2.20× | 1.88× |
| RollingMidprice | [TA-Lib: `MIDPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.68× | 2.01× | 2.11× |
| RollingMin | [TA-Lib: `MIN`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 2.17× | 1.86× |
| RollingMinMax | [TA-Lib: `MINMAX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.83× | 2.03× | 2.18× |
| RollingMinMaxIndex | [TA-Lib: `MINMAXINDEX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.02× | 2.12× | 2.24× |
| RollingMode | [pandas: `pandas.rolling_mode`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingQuantile | [pandas: `pandas.rolling_quantile`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingRank | [pandas: `pandas.rolling_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSharpe | [pandas: `pandas.rolling_sharpe`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSkew | [pandas: `pandas.rolling_skew`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingSortino | [pandas: `pandas.rolling_sortino`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingStandardDeviation | [TA-Lib: `STDDEV`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 2.03× | 1.88× | 1.96× |
| RollingSum | [TA-Lib: `SUM`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 1.96× | 1.93× |
| RollingTimeSeriesForecast | [TA-Lib: `TSF`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.61× | 2.08× | 2.27× |
| RollingVariance | [TA-Lib: `VAR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.87× | 1.86× | 1.98× |
| RollingVolumeWeightedAveragePrice | [pandas: `pandas.rolling_vwap`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingWinsorize | [pandas: `pandas.rolling_winsorize`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| RollingZScore | [pandas: `pandas.rolling_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SchaffTrendCycle | [pandas-ta-classic: `pandas_ta_classic.stc`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SessionVolumeLevels | [pandas: `pandas.session_volume_levels`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Sessions | [smartmoneyconcepts: `smartmoneyconcepts.sessions`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| SignalDelay | [pandas: `pandas.signal_delay`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SignedPower | [NumPy: `numpy.sign/numpy.abs/numpy.power`](https://numpy.org/doc/stable/reference/ufuncs.html) | 0.96× | 1.10× | 1.05× |
| SimpleMovingAverage | [TA-Lib: `SMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.84× | 1.91× | 1.97× |
| SmoothedTrendChannel | [pandas: `pandas.ssl_channel`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| SpreadZScore | [pandas: `pandas.spread_zscore`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| Squeeze | [pandas-ta-classic: `pandas-ta-classic.squeeze`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SqueezePro | [pandas-ta-classic: `pandas-ta-classic.squeeze_pro`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| StochasticOscillator | [TA-Lib: `STOCH`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.74× | 2.16× | 2.18× |
| StochasticRelativeStrengthIndex | [TA-Lib: `STOCHRSI`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.55× | 2.32× | 2.31× |
| Supertrend | [pandas-ta-classic: `pandas-ta-classic.supertrend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| SwingHighsLows | [smartmoneyconcepts: `smartmoneyconcepts.swing_highs_lows`](https://github.com/joshyattridge/smart-money-concepts/tree/1b62fd6c41e1f508e7ed76831a039fa4c82d42f6) | — | — | — |
| TimeSeriesRank | [pandas: `pandas.time_series_rank`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| TomDeMarkSequential | [pandas-ta-classic: `pandas-ta-classic.td_sequential`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| TriangularMovingAverage | [TA-Lib: `TRIMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.77× | 1.96× | 1.90× |
| TripleExponentialAverage | [TA-Lib: `T3`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.88× | 1.91× | 1.99× |
| TripleExponentialMovingAverage | [TA-Lib: `TEMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.80× | 2.05× | 2.10× |
| TripleExponentialRateOfChange | [TA-Lib: `TRIX`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.59× | 2.17× | 2.07× |
| TrueRange | [TA-Lib: `TRANGE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.72× | 1.81× | 1.79× |
| TrueStrengthIndex | [pandas-ta-classic: `pandas-ta-classic.true_strength_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| TypicalPrice | [TA-Lib: `TYPPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.91× | 1.72× | 1.85× |
| UlcerIndex | [pandas-ta-classic: `pandas-ta-classic.ulcer_index`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| UltimateOscillator | [TA-Lib: `ULTOSC`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.76× | 2.08× | 2.10× |
| ValueWhen | [pandas: `pandas.value_when`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| VariableIndexDynamicAverage | [pandas-ta-classic: `pandas-ta-classic.vidya`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| VariablePeriodMovingAverage | [TA-Lib: `MAVP`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.07× | 2.48× | 2.82× |
| VolumePriceTrend | [pandas-ta-classic: `pandas-ta-classic.volume_price_trend`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| VolumeWeightedMovingAverage | [pandas-ta-classic: `pandas_ta_classic.vwma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| Vortex | [pandas-ta-classic: `pandas-ta-classic.vortex`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
| WeightedClose | [TA-Lib: `WCLPRICE`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.73× | 1.91× | 1.89× |
| WeightedMovingAverage | [TA-Lib: `WMA`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.64× | 2.04× | 1.82× |
| WilliamsPercentR | [TA-Lib: `WILLR`](https://ta-lib.github.io/ta-lib-python/funcs.html) | 1.67× | 2.01× | 2.09× |
| YangZhang | [pandas: `pandas.yang_zhang`](https://pandas.pydata.org/docs/reference/window.html) | — | — | — |
| ZeroLagExponentialMovingAverage | [pandas-ta-classic: `pandas_ta_classic.zlma`](https://xgboosted.github.io/pandas-ta-classic/indicators.html) | — | — | — |
