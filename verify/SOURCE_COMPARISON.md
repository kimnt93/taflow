# Source-labelled correctness comparison

Generated from `report.json` and `EXTERNAL_ORACLES.json` by `source_comparison.py`. `INVARIANT` means native lifecycle self-consistency, not external numerical validation.

Matches: **268** | Documented variants: **26** | Self-invariant only: **76** | Failures: **0**

| Python | Function | Output | Source | Verdict | Max error | NaN mismatches | Note |
|---|---|---|---|---:|---:|---:|---|
| `Amihud` | `amihud` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `AnchoredVolumeWeightedAveragePrice` | `anchored_vwap` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `ArnaudLegouxMovingAverage` | `arnaud_legoux_moving_average` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `AverageDailyDollarValue` | `average_daily_dollar_value` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `BarsSince` | `bars_since` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `ChaikinVolatility` | `chaikin_volatility` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `CloseToCloseSigma` | `close_to_close_sigma` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `CumulativeCount` | `cumulative_count` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `CumulativeSumControlChart` | `cumulative_sum_control_chart` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `DecayLinear` | `decay_linear` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `Drawdown` | `drawdown` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `EaseOfMovement` | `ease_of_movement` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `EqualHighsLows` | `equal_highs_lows` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `ExponentiallyWeightedCorrelation` | `ewm_corr` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `ExponentiallyWeightedCovariance` | `ewm_cov` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `ExponentiallyWeightedSum` | `exponentially_weighted_sum` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `Falling` | `falling` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `FibonacciRetracement` | `fib_retracement` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `FracDiff` | `frac_diff` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `FractalDimension` | `fractal_dimension` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `GapDown` | `gap_down` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `GapUp` | `gap_up` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `GarmanKlass` | `garman_klass` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `GarmanKlassYangZhang` | `garman_klass_yang_zhang` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `HedgeRatio` | `hedge_ratio` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `HigherHigh` | `higher_high` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `HighestSince` | `highest_since` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `Hurst` | `hurst` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `InsideBar` | `inside_bar` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `KalmanHedgeRatio` | `kalman_hedge_ratio` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `KeltnerChannels` | `keltner_channels` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `Lag` | `lag` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `LaguerreRelativeStrengthIndex` | `laguerre_rsi` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `LowerLow` | `lower_low` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `LowestSince` | `lowest_since` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `MathAcosh` | `math_acosh` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `MathAsinh` | `math_asinh` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `MathAtanh` | `math_atanh` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `MathCbrt` | `math_cbrt` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `MathCot` | `math_cot` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `MathDegrees` | `math_degrees` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `MathLog1p` | `math_log1p` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `MathRadians` | `math_radians` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `OpeningRange` | `opening_range` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `OrnsteinUhlenbeckHalfLife` | `ornstein_uhlenbeck_half_life` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `OutsideBar` | `outside_bar` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `Parkinson` | `parkinson` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `PivotPoints` | `pivot_points` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `ParabolicMovingAverageStop` | `pmax` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `PremiumDiscount` | `premium_discount` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `Rising` | `rising` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RelativeMomentumIndex` | `rmi` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RogersSatchell` | `rogers_satchell` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollSpread` | `roll_spread` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingAlpha` | `rolling_alpha` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingAutocorr` | `rolling_autocorr` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingCalmar` | `rolling_calmar` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingEntropy` | `rolling_entropy` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingInformationRatio` | `rolling_information_ratio` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingMode` | `rolling_mode` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingRank` | `rolling_rank` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingSharpe` | `rolling_sharpe` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingSortino` | `rolling_sortino` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingVolumeWeightedAveragePrice` | `rolling_vwap` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `RollingWinsorize` | `rolling_winsorize` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `SessionVolumeLevels` | `session_volume_levels` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `SignalDelay` | `signal_delay` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `SignedPower` | `signed_power` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `SpreadZScore` | `spread_zscore` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `SmoothedTrendChannel` | `ssl_channel` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `TimeSeriesRank` | `time_series_rank` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `TrueStrengthIndex` | `true_strength_index` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `UlcerIndex` | `ulcer_index` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `ValueWhen` | `value_when` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `VolumePriceTrend` | `volume_price_trend` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `YangZhang` | `yang_zhang` | `all` | self | INVARIANT | `0.000e+00` | 0 | native batch/append/chunk consistency only; no external oracle |
| `abs` | `abs` | `abs` | Polars | MATCH | `0.000e+00` | 0 |  |
| `ceil` | `ceil` | `ceil` | Polars | MATCH | `0.000e+00` | 0 |  |
| `correlation` | `correlation` | `correlation` | Polars | MATCH | `2.254e-11` | 0 |  |
| `cos` | `cos` | `cos` | Polars | MATCH | `0.000e+00` | 0 |  |
| `covariance` | `covariance` | `covariance` | Polars | MATCH | `5.835e-12` | 0 |  |
| `cumulative_maximum` | `cumulative_maximum` | `cumulative_maximum` | Polars | MATCH | `0.000e+00` | 0 |  |
| `cumulative_minimum` | `cumulative_minimum` | `cumulative_minimum` | Polars | MATCH | `0.000e+00` | 0 |  |
| `cumulative_product` | `cumulative_product` | `cumulative_product` | Polars | MATCH | `7.105e-15` | 0 |  |
| `cumulative_sum` | `cumulative_sum` | `cumulative_sum` | Polars | MATCH | `0.000e+00` | 0 |  |
| `ewm_stddev` | `ewm_stddev` | `ewm_stddev` | Polars | MATCH | `1.776e-15` | 0 |  |
| `ewm_variance` | `ewm_variance` | `ewm_variance` | Polars | MATCH | `1.066e-14` | 0 |  |
| `exp` | `exp` | `exp` | Polars | MATCH | `4.441e-16` | 0 |  |
| `floor` | `floor` | `floor` | Polars | MATCH | `0.000e+00` | 0 |  |
| `kurtosis` | `kurtosis` | `kurtosis` | Polars | MATCH | `4.695e-05` | 0 | Polars raw-moment kernel tolerance after centering/scaling |
| `ln` | `ln` | `ln` | Polars | MATCH | `0.000e+00` | 0 |  |
| `max` | `max` | `max` | Polars | MATCH | `0.000e+00` | 0 |  |
| `mean` | `mean` | `mean` | Polars | MATCH | `2.274e-13` | 0 |  |
| `median` | `median` | `median` | Polars | MATCH | `0.000e+00` | 0 |  |
| `min` | `min` | `min` | Polars | MATCH | `0.000e+00` | 0 |  |
| `quantile` | `quantile` | `quantile` | Polars | MATCH | `0.000e+00` | 0 |  |
| `sin` | `sin` | `sin` | Polars | MATCH | `0.000e+00` | 0 |  |
| `skew` | `skew` | `skew` | Polars | MATCH | `8.776e-08` | 0 | Polars raw-moment kernel tolerance after centering/scaling |
| `sqrt` | `sqrt` | `sqrt` | Polars | MATCH | `0.000e+00` | 0 |  |
| `stddev` | `stddev` | `stddev` | Polars | MATCH | `2.402e-11` | 0 |  |
| `sum` | `sum` | `sum` | Polars | MATCH | `3.070e-12` | 0 |  |
| `tan` | `tan` | `tan` | Polars | MATCH | `0.000e+00` | 0 |  |
| `variance` | `variance` | `variance` | Polars | MATCH | `3.930e-11` | 0 |  |
| `AccelerationBands` | `ACCBANDS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathAcos` | `ACOS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `AccumulationDistribution` | `AD` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathAdd` | `ADD` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `AccumulationDistributionOscillator` | `ADOSC` | `all` | TA-Lib | MATCH | `1.490e-08` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `AverageDirectionalIndex` | `ADX` | `all` | TA-Lib | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `AverageDirectionalIndexRating` | `ADXR` | `all` | TA-Lib | MATCH | `2.132e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `AbsolutePriceOscillator` | `APO` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `Aroon` | `AROON` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `AroonOscillator` | `AROONOSC` | `all` | TA-Lib | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathAsin` | `ASIN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathAtan` | `ATAN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `AverageTrueRange` | `ATR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingAverageDeviation` | `AVGDEV` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `AveragePrice` | `AVGPRICE` | `all` | TA-Lib | MATCH | `5.684e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `BollingerBands` | `BBANDS` | `all` | TA-Lib | MATCH | `7.927e-10` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingBeta` | `BETA` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `BalanceOfPower` | `BOP` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CommodityChannelIndex` | `CCI` | `all` | TA-Lib | MATCH | `1.387e-11` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleTwoCrows` | `CDL2CROWS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleThreeBlackCrows` | `CDL3BLACKCROWS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleThreeInside` | `CDL3INSIDE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleThreeLineStrike` | `CDL3LINESTRIKE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleThreeOutside` | `CDL3OUTSIDE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleThreeStarsInSouth` | `CDL3STARSINSOUTH` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleThreeWhiteSoldiers` | `CDL3WHITESOLDIERS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleAbandonedBaby` | `CDLABANDONEDBABY` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleAdvanceBlock` | `CDLADVANCEBLOCK` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleBeltHold` | `CDLBELTHOLD` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleBreakaway` | `CDLBREAKAWAY` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleClosingMarubozu` | `CDLCLOSINGMARUBOZU` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleConcealBabySwall` | `CDLCONCEALBABYSWALL` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleCounterAttack` | `CDLCOUNTERATTACK` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleDarkCloudCover` | `CDLDARKCLOUDCOVER` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleDoji` | `CDLDOJI` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleDojiStar` | `CDLDOJISTAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleDragonflyDoji` | `CDLDRAGONFLYDOJI` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleEngulfing` | `CDLENGULFING` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleEveningDojiStar` | `CDLEVENINGDOJISTAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleEveningStar` | `CDLEVENINGSTAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleGapSideSideWhite` | `CDLGAPSIDESIDEWHITE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleGravestoneDoji` | `CDLGRAVESTONEDOJI` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleHammer` | `CDLHAMMER` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleHangingMan` | `CDLHANGINGMAN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleHarami` | `CDLHARAMI` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleHaramiCross` | `CDLHARAMICROSS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleHighWave` | `CDLHIGHWAVE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleHikkake` | `CDLHIKKAKE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleHikkakeModified` | `CDLHIKKAKEMOD` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleHomingPigeon` | `CDLHOMINGPIGEON` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleIdenticalThreeCrows` | `CDLIDENTICAL3CROWS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleInNeck` | `CDLINNECK` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleInvertedHammer` | `CDLINVERTEDHAMMER` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleKicking` | `CDLKICKING` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleKickingByLength` | `CDLKICKINGBYLENGTH` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleLadderBottom` | `CDLLADDERBOTTOM` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleLongLeggedDoji` | `CDLLONGLEGGEDDOJI` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleLongLine` | `CDLLONGLINE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleMarubozu` | `CDLMARUBOZU` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleMatchingLow` | `CDLMATCHINGLOW` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleMatHold` | `CDLMATHOLD` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleMorningDojiStar` | `CDLMORNINGDOJISTAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleMorningStar` | `CDLMORNINGSTAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleOnNeck` | `CDLONNECK` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandlePiercing` | `CDLPIERCING` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleRickshawman` | `CDLRICKSHAWMAN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleRiseFallThreeMethods` | `CDLRISEFALL3METHODS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleSeparatingLines` | `CDLSEPARATINGLINES` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleShootingStar` | `CDLSHOOTINGSTAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleShortLine` | `CDLSHORTLINE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleSpinningTop` | `CDLSPINNINGTOP` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleStalledPattern` | `CDLSTALLEDPATTERN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleStickSandwich` | `CDLSTICKSANDWICH` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleTakuri` | `CDLTAKURI` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleTasukiGap` | `CDLTASUKIGAP` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleThrusting` | `CDLTHRUSTING` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleTriStar` | `CDLTRISTAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleUniqueThreeRiver` | `CDLUNIQUE3RIVER` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleUpsideGapTwoCrows` | `CDLUPSIDEGAP2CROWS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `CandleUpDownSideGapThreeMethods` | `CDLXSIDEGAP3METHODS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathCeil` | `CEIL` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `ChandeMomentumOscillator` | `CMO` | `all` | TA-Lib | MATCH | `6.817e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingCorrelation` | `CORREL` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathCos` | `COS` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathCosh` | `COSH` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `DoubleExponentialMovingAverage` | `DEMA` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathDivide` | `DIV` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `DirectionalMovementIndex` | `DX` | `all` | TA-Lib | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `ExponentialMovingAverage` | `EMA` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathExp` | `EXP` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathFloor` | `FLOOR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `HilbertTransformDominantCyclePeriod` | `HT_DCPERIOD` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `HilbertTransformDominantCyclePhase` | `HT_DCPHASE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `HilbertTransformPhasor` | `HT_PHASOR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `HilbertTransformSineWave` | `HT_SINE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `HilbertTransformTrendline` | `HT_TRENDLINE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `HilbertTransformTrendMode` | `HT_TRENDMODE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `IntradayMomentumIndex` | `IMI` | `all` | TA-Lib | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `KaufmanAdaptiveMovingAverage` | `KAMA` | `all` | TA-Lib | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingLinearRegression` | `LINEARREG` | `all` | TA-Lib | MATCH | `5.969e-13` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingLinearRegressionAngle` | `LINEARREG_ANGLE` | `all` | TA-Lib | MATCH | `4.924e-12` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingLinearRegressionIntercept` | `LINEARREG_INTERCEPT` | `all` | TA-Lib | MATCH | `5.969e-13` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingLinearRegressionSlope` | `LINEARREG_SLOPE` | `all` | TA-Lib | MATCH | `9.137e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathLn` | `LN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathLog10` | `LOG10` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MovingAverage` | `MA` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MovingAverageConvergenceDivergence` | `MACD` | `all` | TA-Lib | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MovingAverageConvergenceDivergenceExtended` | `MACDEXT` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MovingAverageConvergenceDivergenceFixed` | `MACDFIX` | `all` | TA-Lib | MATCH | `5.684e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MesaAdaptiveMovingAverage` | `MAMA` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `VariablePeriodMovingAverage` | `MAVP` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingMax` | `MAX` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingArgmax` | `MAXINDEX` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MedianPrice` | `MEDPRICE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MoneyFlowIndex` | `MFI` | `all` | TA-Lib | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingMidpoint` | `MIDPOINT` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingMidprice` | `MIDPRICE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingMin` | `MIN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingArgmin` | `MININDEX` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingMinMax` | `MINMAX` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingMinMaxIndex` | `MINMAXINDEX` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MinusDirectionalIndicator` | `MINUS_DI` | `all` | TA-Lib | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MinusDirectionalMovement` | `MINUS_DM` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `Momentum` | `MOM` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathMultiply` | `MULT` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `NormalizedAverageTrueRange` | `NATR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `OnBalanceVolume` | `OBV` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `PlusDirectionalIndicator` | `PLUS_DI` | `all` | TA-Lib | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `PlusDirectionalMovement` | `PLUS_DM` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `PercentagePriceOscillator` | `PPO` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RateOfChange` | `ROC` | `all` | TA-Lib | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RateOfChangePercent` | `ROCP` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RateOfChangeRatio` | `ROCR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RateOfChangeRatioPercent` | `ROCR100` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RelativeStrengthIndex` | `RSI` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `ParabolicSar` | `SAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `ParabolicSarExtended` | `SAREXT` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathSin` | `SIN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathSinh` | `SINH` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `SimpleMovingAverage` | `SMA` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathSqrt` | `SQRT` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingStandardDeviation` | `STDDEV` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `StochasticOscillator` | `STOCH` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `FastStochasticOscillator` | `STOCHF` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `StochasticRelativeStrengthIndex` | `STOCHRSI` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathSubtract` | `SUB` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingSum` | `SUM` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `TripleExponentialAverage` | `T3` | `all` | TA-Lib | MATCH | `5.684e-13` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathTan` | `TAN` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `MathTanh` | `TANH` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `TripleExponentialMovingAverage` | `TEMA` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `TrueRange` | `TRANGE` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `TriangularMovingAverage` | `TRIMA` | `all` | TA-Lib | MATCH | `6.821e-13` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `TripleExponentialRateOfChange` | `TRIX` | `all` | TA-Lib | MATCH | `1.110e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingTimeSeriesForecast` | `TSF` | `all` | TA-Lib | MATCH | `6.821e-13` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `TypicalPrice` | `TYPPRICE` | `all` | TA-Lib | MATCH | `5.684e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `UltimateOscillator` | `ULTOSC` | `all` | TA-Lib | MATCH | `1.421e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingVariance` | `VAR` | `all` | TA-Lib | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `WeightedClose` | `WCLPRICE` | `all` | TA-Lib | MATCH | `5.684e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `WilliamsPercentR` | `WILLR` | `all` | TA-Lib | MATCH | `2.842e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `WeightedMovingAverage` | `WMA` | `all` | TA-Lib | MATCH | `1.857e-10` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `ExponentiallyWeightedStandardDeviation` | `ewm_std` | `all` | pandas | MATCH | `7.816e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `ExponentiallyWeightedVariance` | `ewm_var` | `all` | pandas | MATCH | `1.904e-12` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingCov` | `rolling_cov` | `all` | pandas | MATCH | `1.282e-11` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingKurtosis` | `rolling_kurtosis` | `all` | pandas | MATCH | `1.332e-15` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingMedian` | `rolling_median` | `all` | pandas | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingQuantile` | `rolling_quantile` | `all` | pandas | MATCH | `0.000e+00` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingSkew` | `rolling_skew` | `all` | pandas | MATCH | `8.882e-16` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `RollingZScore` | `rolling_zscore` | `all` | pandas | MATCH | `4.771e-14` | 0 | external parity plus bitwise lifecycle/chunk invariance |
| `awesome_oscillator` | `awesome_oscillator` | `ao` | pandas-ta-classic | MATCH | `8.527e-14` | 0 |  |
| `chaikin_money_flow` | `chaikin_money_flow` | `cmf` | pandas-ta-classic | MATCH | `3.993e-14` | 0 |  |
| `crossover` | `crossover` | `crossover` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `crossunder` | `crossunder` | `crossunder` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `detrended_price_oscillator` | `detrended_price_oscillator` | `dpo` | pandas-ta-classic | MATCH | `1.137e-13` | 0 |  |
| `donchian_channels` | `donchian_channels` | `lower` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `donchian_channels` | `donchian_channels` | `mid` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `donchian_channels` | `donchian_channels` | `upper` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `even_better_sinewave` | `even_better_sinewave` | `ebsw` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `even_better_sinewave` | `even_better_sinewave` | `ebsw[length=60]` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `fisher_transform` | `fisher_transform` | `fisher` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `fisher_transform` | `fisher_transform` | `fisher[length=11]` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `fisher_transform` | `fisher_transform` | `fisher[length=21]` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `force_index` | `force_index` | `force_index` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | taflow exposes the unsmoothed one-bar force; pandas-ta EFI length=1 |
| `heikin_ashi` | `heikin_ashi` | `close` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `heikin_ashi` | `heikin_ashi` | `high` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `heikin_ashi` | `heikin_ashi` | `low` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `heikin_ashi` | `heikin_ashi` | `open` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `hull_moving_average` | `hull_moving_average` | `hma` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `ichimoku` | `ichimoku` | `chikou` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `ichimoku` | `ichimoku` | `kijun` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `ichimoku` | `ichimoku` | `span_a` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `ichimoku` | `ichimoku` | `span_b` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `ichimoku` | `ichimoku` | `tenkan` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| `jurik_moving_average` | `jurik_moving_average` | `jma` | pandas-ta-classic | MATCH | `2.842e-14` | 0 |  |
| `jurik_moving_average` | `jurik_moving_average` | `jma[length=1,phase=0]` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `jurik_moving_average` | `jurik_moving_average` | `jma[length=2,phase=-100]` | pandas-ta-classic | MATCH | `1.421e-14` | 0 |  |
| `jurik_moving_average` | `jurik_moving_average` | `jma[length=21,phase=35]` | pandas-ta-classic | MATCH | `2.842e-14` | 0 |  |
| `jurik_moving_average` | `jurik_moving_average` | `jma[length=7,phase=100]` | pandas-ta-classic | MATCH | `4.263e-14` | 0 |  |
| `klinger_volume_oscillator` | `klinger_volume_oscillator` | `kvo` | pandas-ta-classic | MATCH | `2.183e-10` | 0 |  |
| `klinger_volume_oscillator` | `klinger_volume_oscillator` | `kvo[fast=5,slow=8,signal=3]` | pandas-ta-classic | MATCH | `2.910e-10` | 0 |  |
| `klinger_volume_oscillator` | `klinger_volume_oscillator` | `signal` | pandas-ta-classic | MATCH | `1.914e-10` | 0 |  |
| `klinger_volume_oscillator` | `klinger_volume_oscillator` | `signal[fast=5,slow=8,signal=3]` | pandas-ta-classic | MATCH | `2.146e-10` | 0 |  |
| `log_return` | `log_return` | `log_return` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `mcginley_dynamic` | `mcginley_dynamic` | `mcginley` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `schaff_trend_cycle` | `schaff_trend_cycle` | `macd` | pandas-ta-classic | MATCH | `7.105e-14` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| `schaff_trend_cycle` | `schaff_trend_cycle` | `stc` | pandas-ta-classic | MATCH | `1.000e-08` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| `schaff_trend_cycle` | `schaff_trend_cycle` | `stochastic` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| `squeeze` | `squeeze` | `momentum` | pandas-ta-classic | MATCH | `3.553e-15` | 0 |  |
| `squeeze` | `squeeze` | `no` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `squeeze` | `squeeze` | `off` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `squeeze` | `squeeze` | `on` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `squeeze_pro` | `squeeze_pro` | `momentum` | pandas-ta-classic | MATCH | `3.553e-15` | 0 |  |
| `squeeze_pro` | `squeeze_pro` | `no` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `squeeze_pro` | `squeeze_pro` | `off` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `squeeze_pro` | `squeeze_pro` | `on_narrow` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `squeeze_pro` | `squeeze_pro` | `on_normal` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `squeeze_pro` | `squeeze_pro` | `on_wide` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `supertrend` | `supertrend` | `direction` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| `supertrend` | `supertrend` | `long` | pandas-ta-classic | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| `supertrend` | `supertrend` | `short` | pandas-ta-classic | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| `supertrend` | `supertrend` | `trend` | pandas-ta-classic | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| `tom_de_mark_sequential` | `tom_de_mark_sequential` | `buy` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | pandas-ta columns reordered and capped at the DeMark setup count of nine |
| `tom_de_mark_sequential` | `tom_de_mark_sequential` | `sell` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | pandas-ta columns reordered and capped at the DeMark setup count of nine |
| `variable_index_dynamic_average` | `variable_index_dynamic_average` | `vidya` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `variable_index_dynamic_average` | `variable_index_dynamic_average` | `vidya[length=1]` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `variable_index_dynamic_average` | `variable_index_dynamic_average` | `vidya[length=2]` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `variable_index_dynamic_average` | `variable_index_dynamic_average` | `vidya[length=30]` | pandas-ta-classic | MATCH | `1.421e-14` | 0 |  |
| `volume_weighted_moving_average` | `volume_weighted_moving_average` | `vwma` | pandas-ta-classic | MATCH | `5.684e-14` | 0 |  |
| `vortex` | `vortex` | `minus` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `vortex` | `vortex` | `plus` | pandas-ta-classic | MATCH | `0.000e+00` | 0 |  |
| `zero_lag_exponential_moving_average` | `zero_lag_exponential_moving_average` | `zlema` | pandas-ta-classic | MATCH | `7.105e-14` | 0 | force pandas-ta's native EMA; TA-Lib rejects leading ZLMA NaNs |
| `zero_lag_exponential_moving_average` | `zero_lag_exponential_moving_average` | `zlema[length=1]` | pandas-ta-classic | MATCH | `0.000e+00` | 0 | parameter matrix; force pandas-ta's native EMA |
| `zero_lag_exponential_moving_average` | `zero_lag_exponential_moving_average` | `zlema[length=21]` | pandas-ta-classic | MATCH | `7.105e-14` | 0 | parameter matrix; force pandas-ta's native EMA |
| `zero_lag_exponential_moving_average` | `zero_lag_exponential_moving_average` | `zlema[length=2]` | pandas-ta-classic | MATCH | `1.421e-14` | 0 | parameter matrix; force pandas-ta's native EMA |
| `fair_value_gap` | `fair_value_gap` | `bottom` | smartmoneyconcepts | MATCH | `0.000e+00` | 0 | package marker at middle candle; shifted to causal detection bar |
| `fair_value_gap` | `fair_value_gap` | `fvg` | smartmoneyconcepts | MATCH | `0.000e+00` | 0 | package marker at middle candle; shifted to causal detection bar |
| `fair_value_gap` | `fair_value_gap` | `mitigated` | smartmoneyconcepts | MATCH | `0.000e+00` | 0 | package future index converted to flag at mitigation bar |
| `fair_value_gap` | `fair_value_gap` | `top` | smartmoneyconcepts | MATCH | `0.000e+00` | 0 | package marker at middle candle; shifted to causal detection bar |
| `sessions` | `sessions` | `active` | smartmoneyconcepts | MATCH | `0.000e+00` | 0 | all-day custom session; boundaries passed as flags |
| `sessions` | `sessions` | `high` | smartmoneyconcepts | MATCH | `3.589e-06` | 0 | all-day custom session; boundaries passed as flags |
| `sessions` | `sessions` | `low` | smartmoneyconcepts | MATCH | `3.808e-06` | 0 | all-day custom session; boundaries passed as flags |
| `know_sure_thing` | `know_sure_thing` | `kst` | pandas-ta-classic | VARIANT | `1.824e+04` | 0 | taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100 |
| `know_sure_thing` | `know_sure_thing` | `signal` | pandas-ta-classic | VARIANT | `1.782e+04` | 8 | taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100 |
| `mass_index` | `mass_index` | `mass` | pandas-ta-classic | VARIANT | `4.737e-02` | 0 | taflow follows bukosabino/ta EMA initialization |
| `negative_volume_index` | `negative_volume_index` | `nvi` | pandas-ta-classic | VARIANT | `5.278e+02` | 0 | taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC |
| `positive_volume_index` | `positive_volume_index` | `pvi` | pandas-ta-classic | VARIANT | `4.481e+02` | 0 | taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC |
| `break_of_structure_change_of_character` | `break_of_structure_change_of_character` | `bos` | smartmoneyconcepts | VARIANT | `0.000e+00` | 71 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| `break_of_structure_change_of_character` | `break_of_structure_change_of_character` | `broken` | smartmoneyconcepts | VARIANT | `0.000e+00` | 101 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| `break_of_structure_change_of_character` | `break_of_structure_change_of_character` | `choch` | smartmoneyconcepts | VARIANT | `0.000e+00` | 30 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| `break_of_structure_change_of_character` | `break_of_structure_change_of_character` | `level` | smartmoneyconcepts | VARIANT | `0.000e+00` | 101 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| `liquidity` | `liquidity` | `level` | smartmoneyconcepts | VARIANT | `1.500e+00` | 184 | package uses full-series range and retroactive group starts; taflow uses causal level-relative tolerance and emits second-touch/sweep events |
| `liquidity` | `liquidity` | `liquidity` | smartmoneyconcepts | VARIANT | `2.000e+00` | 125 | package uses full-series range and retroactive group starts; taflow uses causal level-relative tolerance and emits second-touch/sweep events |
| `liquidity` | `liquidity` | `swept` | smartmoneyconcepts | VARIANT | `0.000e+00` | 71 | package uses full-series range and retroactive group starts; taflow uses causal level-relative tolerance and emits second-touch/sweep events |
| `order_block` | `order_block` | `bottom` | smartmoneyconcepts | VARIANT | `0.000e+00` | 170 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| `order_block` | `order_block` | `mitigated` | smartmoneyconcepts | VARIANT | `0.000e+00` | 117 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| `order_block` | `order_block` | `ob` | smartmoneyconcepts | VARIANT | `0.000e+00` | 170 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| `order_block` | `order_block` | `ob_volume` | smartmoneyconcepts | VARIANT | `0.000e+00` | 170 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| `order_block` | `order_block` | `top` | smartmoneyconcepts | VARIANT | `0.000e+00` | 170 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| `previous_high_low` | `previous_high_low` | `broken_high` | smartmoneyconcepts | VARIANT | `1.000e+00` | 989 | SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day |
| `previous_high_low` | `previous_high_low` | `broken_low` | smartmoneyconcepts | VARIANT | `1.000e+00` | 987 | SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day |
| `previous_high_low` | `previous_high_low` | `previous_high` | smartmoneyconcepts | VARIANT | `4.564e+01` | 96 | SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day |
| `previous_high_low` | `previous_high_low` | `previous_low` | smartmoneyconcepts | VARIANT | `3.688e+01` | 96 | SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day |
| `retracements` | `retracements` | `current_retracement_pct` | smartmoneyconcepts | VARIANT | `6.093e+02` | 60 | package is lookahead-aligned, rounded, and uses candle extremes; taflow confirms swings causally and measures the current close |
| `retracements` | `retracements` | `deepest_retracement_pct` | smartmoneyconcepts | VARIANT | `6.093e+02` | 60 | package is lookahead-aligned, rounded, and uses candle extremes; taflow confirms swings causally and measures the current close |
| `retracements` | `retracements` | `direction` | smartmoneyconcepts | VARIANT | `2.000e+00` | 14 | package is lookahead-aligned, rounded, and uses candle extremes; taflow confirms swings causally and measures the current close |
| `swing_highs_lows` | `swing_highs_lows` | `level` | smartmoneyconcepts | VARIANT | `0.000e+00` | 41 | package also removes markers retroactively; causal taflow cannot retract emitted events |
| `swing_highs_lows` | `swing_highs_lows` | `signal` | smartmoneyconcepts | VARIANT | `0.000e+00` | 41 | package also removes markers retroactively; causal taflow cannot retract emitted events |
