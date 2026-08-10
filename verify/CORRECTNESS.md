# TAFlow correctness verification

Date: 2026-08-10 | bars: 1,000 | warm-up split: 9,000 extend + -8,000 append | tolerance rtol=1e-08, atol=1e-10
Environment: python 3.12.3, numpy 2.4.6, TA-Lib 0.7.1, Wickra 0.9.9, SMC 0.0.27, TAFlow 0.1.2

Summary: ERROR: 11, FAIL: 42, MATCH: 218, NO_EXTERNAL_ORACLE: 121, VARIANT: 1

TAFlow is driven only through canonical Python classes. The registry
selects TA-Lib, Wickra, explicit NumPy ufunc overrides, then SMC.
*Batch vs oracle*:
cold `extend` over the full series against the reference;
*continue vs batch*: 9k `extend` + 1k `append` stitched output
bitwise-identical to one-shot batch (chunk invariance); *continue
vs oracle*: the stitched output against the reference. Repeated
native `extend` chunks [1, 10, 1000] are also checked bitwise.

| Function | taflow class | Oracle | Verdict | Batch vs oracle | Continue vs batch | Extend chunks | Continue vs oracle |
|---|---|---|---|---|---|---|---|
| absolute_breadth_index | AbsoluteBreadthIndex | Wickra | ERROR | — | — | yes | — |
| adaptive_cycle | AdaptiveCycle | Wickra | FAIL | **FAIL** (err 1.6e+01, nan 48) | yes | yes | **FAIL** (err 1.6e+01, nan 48) |
| amihud | Amihud | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| anchored_vwap | AnchoredVolumeWeightedAveragePrice | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| arms_index | ArmsIndex | Wickra | ERROR | — | — | yes | — |
| arnaud_legoux_moving_average | ArnaudLegouxMovingAverage | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| automatic_fibonacci | AutomaticFibonacci | Wickra | FAIL | **FAIL** (err 5.8e+01, nan 70) | yes | yes | **FAIL** (err 5.8e+01, nan 70) |
| average_daily_dollar_value | AverageDailyDollarValue | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| average_true_range_bands | AverageTrueRangeBands | Wickra | FAIL | error: output arity 1 != 3 | yes | yes | error: output arity 1 != 3 |
| awesome_oscillator | AwesomeOscillator | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| bars_since | BarsSince | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| bat_pattern | BatPattern | Wickra | FAIL | **FAIL** (err 0.0e+00, nan 5) | yes | yes | **FAIL** (err 0.0e+00, nan 5) |
| breadth_thrust | BreadthThrust | Wickra | ERROR | — | — | yes | — |
| break_of_structure_change_of_character | BreakOfStructureChangeOfCharacter | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| bullish_percent_index | BullishPercentIndex | Wickra | ERROR | — | — | yes | — |
| butterfly_pattern | ButterflyPattern | Wickra | FAIL | **FAIL** (err 1.0e+00, nan 5) | yes | yes | **FAIL** (err 1.0e+00, nan 5) |
| chaikin_money_flow | ChaikinMoneyFlow | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| chaikin_volatility | ChaikinVolatility | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| close_to_close_sigma | CloseToCloseSigma | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| crab_pattern | CrabPattern | Wickra | FAIL | **FAIL** (err 0.0e+00, nan 5) | yes | yes | **FAIL** (err 0.0e+00, nan 5) |
| cross | Cross | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| crossover | Crossover | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| crossunder | Crossunder | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| cumulative_count | CumulativeCount | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| cumulative_maximum | CumulativeMaximum | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| cumulative_minimum | CumulativeMinimum | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| cumulative_product | CumulativeProduct | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| cumulative_sum | CumulativeSum | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| cumulative_sum_control_chart | CumulativeSumControlChart | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| cumulative_volume_index | CumulativeVolumeIndex | Wickra | ERROR | — | — | yes | — |
| cup_and_handle | CupAndHandle | Wickra | FAIL | **FAIL** (err 2.0e+00, nan 19) | yes | yes | **FAIL** (err 2.0e+00, nan 19) |
| cypher_pattern | CypherPattern | Wickra | FAIL | **FAIL** (err 1.0e+00, nan 5) | yes | yes | **FAIL** (err 1.0e+00, nan 5) |
| day_of_week_return_profile | DayOfWeekReturnProfile | Wickra | FAIL | error: output arity 1 != 7 | yes | yes | error: output arity 1 != 7 |
| decay_linear | DecayLinear | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| detrended_price_oscillator | DetrendedPriceOscillator | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| donchian | Donchian | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| double_bollinger_bands | DoubleBollingerBands | Wickra | FAIL | error: output arity 1 != 5 | yes | yes | error: output arity 1 != 5 |
| drawdown | Drawdown | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| ease_of_movement | EaseOfMovement | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| empirical_mode_decomposition | EmpiricalModeDecomposition | Wickra | FAIL | **FAIL** (err 6.1e+01, nan 10) | yes | yes | **FAIL** (err 6.1e+01, nan 10) |
| entry_exit | EntryExit | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| equal_highs_lows | EqualHighsLows | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| even_better_sinewave | EvenBetterSinewave | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| ewm_corr | ExponentiallyWeightedCorrelation | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| ewm_cov | ExponentiallyWeightedCovariance | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| ewm_std | ExponentiallyWeightedStandardDeviation | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| ewm_var | ExponentiallyWeightedVariance | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| exponentially_weighted_sum | ExponentiallyWeightedSum | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| falling | Falling | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| fib_retracement | FibonacciRetracement | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| fibonacci_arcs | FibonacciArcs | Wickra | FAIL | **FAIL** (err 1.0e+02, nan 30) | yes | yes | **FAIL** (err 1.0e+02, nan 30) |
| fibonacci_channel | FibonacciChannel | Wickra | FAIL | **FAIL** (err 7.0e+01, nan 144) | yes | yes | **FAIL** (err 7.0e+01, nan 144) |
| fibonacci_confluence | FibonacciConfluence | Wickra | FAIL | **FAIL** (err 1.4e+02, nan 72) | yes | yes | **FAIL** (err 1.4e+02, nan 72) |
| fibonacci_extension | FibonacciExtension | Wickra | FAIL | **FAIL** (err 2.5e+02, nan 50) | yes | yes | **FAIL** (err 2.5e+02, nan 50) |
| fibonacci_fan | FibonacciFan | Wickra | FAIL | **FAIL** (err 6.4e+01, nan 30) | yes | yes | **FAIL** (err 6.4e+01, nan 30) |
| fibonacci_projection | FibonacciProjection | Wickra | FAIL | **FAIL** (err 2.4e+02, nan 144) | yes | yes | **FAIL** (err 2.4e+02, nan 144) |
| fibonacci_time_zones | FibonacciTimeZones | Wickra | FAIL | **FAIL** (err 1.0e+03, nan 8) | yes | yes | **FAIL** (err 1.0e+03, nan 8) |
| fisher_transform | FisherTransform | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| flag_pennant | FlagPennant | Wickra | FAIL | **FAIL** (err 2.0e+00, nan 19) | yes | yes | **FAIL** (err 2.0e+00, nan 19) |
| force_index | ForceIndex | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| four_point_harmonic_pattern | FourPointHarmonicPattern | Wickra | FAIL | **FAIL** (err 1.0e+00, nan 4) | yes | yes | **FAIL** (err 1.0e+00, nan 4) |
| frac_diff | FracDiff | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| fractal_dimension | FractalDimension | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| gap_down | GapDown | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| gap_up | GapUp | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| garman_klass | GarmanKlass | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| garman_klass_yang_zhang | GarmanKlassYangZhang | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| gartley_pattern | GartleyPattern | Wickra | FAIL | **FAIL** (err 0.0e+00, nan 5) | yes | yes | **FAIL** (err 0.0e+00, nan 5) |
| golden_pocket | GoldenPocket | Wickra | FAIL | **FAIL** (err 3.7e+01, nan 30) | yes | yes | **FAIL** (err 3.7e+01, nan 30) |
| head_and_shoulders | HeadAndShoulders | Wickra | FAIL | **FAIL** (err 1.0e+00, nan 5) | yes | yes | **FAIL** (err 1.0e+00, nan 5) |
| hedge_ratio | HedgeRatio | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| heikin_ashi | HeikinAshi | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| high_low_index | HighLowIndex | Wickra | ERROR | — | — | yes | — |
| higher_high | HigherHigh | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| highest_since | HighestSince | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| hilbert_dominant_cycle | HilbertDominantCycle | Wickra | FAIL | **FAIL** (err 1.6e+01, nan 17) | yes | yes | **FAIL** (err 1.6e+01, nan 17) |
| hull_moving_average | HullMovingAverage | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| hurst | Hurst | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| hurst_channel | HurstChannel | Wickra | FAIL | error: output arity 1 != 3 | yes | yes | error: output arity 1 != 3 |
| ichimoku | Ichimoku | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| inside_bar | InsideBar | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| intraday_volatility_profile | IntradayVolatilityProfile | Wickra | FAIL | error: output arity 1 != 24 | yes | yes | error: output arity 1 != 24 |
| jma | JurikMovingAverage | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| kalman_hedge_ratio | KalmanHedgeRatio | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| keltner_channels | KeltnerChannels | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| know_sure_thing | KnowSureThing | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| kvo | KlingerVolumeOscillator | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| lag | Lag | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| laguerre_rsi | LaguerreRelativeStrengthIndex | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| linear_regression_channel | LinearRegressionChannel | Wickra | FAIL | error: output arity 1 != 3 | yes | yes | error: output arity 1 != 3 |
| liquidity | Liquidity | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| log_return | LogReturn | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| lower_low | LowerLow | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| lowest_since | LowestSince | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| mass_index | MassIndex | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| mc_clellan_oscillator | McClellanOscillator | Wickra | ERROR | — | — | yes | — |
| mc_clellan_summation_index | McClellanSummationIndex | Wickra | ERROR | — | — | yes | — |
| mcginley | McGinleyDynamic | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| moving_average_envelope | MovingAverageEnvelope | Wickra | FAIL | error: output arity 1 != 3 | yes | yes | error: output arity 1 != 3 |
| negative_volume_index | NegativeVolumeIndex | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| new_highs_new_lows | NewHighsNewLows | Wickra | ERROR | — | — | yes | — |
| opening_range | OpeningRange | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| order_block | OrderBlock | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| ornstein_uhlenbeck_half_life | OrnsteinUhlenbeckHalfLife | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| outside_bar | OutsideBar | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| overnight_gap | OvernightGap | Wickra | FAIL | **FAIL** (err 0.0e+00, nan 956) | yes | yes | **FAIL** (err 0.0e+00, nan 956) |
| parkinson | Parkinson | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| percent_above_moving_average | PercentAboveMovingAverage | Wickra | ERROR | — | — | yes | — |
| pivot_points | PivotPoints | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| pmax | ParabolicMovingAverageStop | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| position_hold | PositionHold | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| positive_volume_index | PositiveVolumeIndex | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| premium_discount | PremiumDiscount | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| previous_high_low | PreviousHighLow | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| projection_bands | ProjectionBands | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rectangle_range | RectangleRange | Wickra | FAIL | **FAIL** (err 1.0e+00, nan 19) | yes | yes | **FAIL** (err 1.0e+00, nan 19) |
| retracements | Retracements | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rising | Rising | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rogers_satchell | RogersSatchell | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| roll_spread | RollSpread | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_alpha | RollingAlpha | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_autocorr | RollingAutocorr | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_calmar | RollingCalmar | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_coefficient_of_determination | RollingCoefficientOfDetermination | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_cointegration | RollingCointegration | Wickra | FAIL | error: output arity 1 != 3 | yes | yes | error: output arity 1 != 3 |
| rolling_covariance | RollingCovariance | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_entropy | RollingEntropy | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_information_ratio | RollingInformationRatio | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_interquartile_range | RollingInterquartileRange | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_kurtosis | RollingKurtosis | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_lead_lag_cross_correlation | RollingLeadLagCrossCorrelation | Wickra | FAIL | error: output arity 1 != 2 | yes | yes | error: output arity 1 != 2 |
| rolling_median | RollingMedian | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_mode | RollingMode | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_percentile | RollingPercentile | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_quantile | RollingQuantile | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_rank | RollingRank | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_recovery_factor | RollingRecoveryFactor | Wickra | VARIANT | **FAIL** (err 1.1e+03, nan 13) | yes | yes | **FAIL** (err 1.1e+03, nan 13) |
| rolling_sharpe | RollingSharpe | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_skew | RollingSkew | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_sortino | RollingSortino | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_vwap | RollingVolumeWeightedAveragePrice | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_winsorize | RollingWinsorize | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| rolling_zscore | RollingZScore | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| schaff_trend_cycle | SchaffTrendCycle | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| session_extrema | SessionExtrema | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| session_volume_levels | SessionVolumeLevels | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| shark_pattern | SharkPattern | Wickra | FAIL | **FAIL** (err 1.0e+00, nan 5) | yes | yes | **FAIL** (err 1.0e+00, nan 5) |
| signal_delay | SignalDelay | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| spread_zscore | SpreadZScore | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| squeeze | Squeeze | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| squeeze_pro | SqueezePro | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| ssl_channel | SmoothedTrendChannel | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| standard_error_bands | StandardErrorBands | Wickra | FAIL | error: output arity 1 != 3 | yes | yes | error: output arity 1 != 3 |
| supertrend | Supertrend | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| swing_high_low | SwingHighLow | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| td_sequential | TomDeMarkSequential | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| three_drives | ThreeDrives | Wickra | FAIL | **FAIL** (err 1.0e+00, nan 5) | yes | yes | **FAIL** (err 1.0e+00, nan 5) |
| time_of_day_return_profile | TimeOfDayReturnProfile | Wickra | FAIL | error: output arity 1 != 24 | yes | yes | error: output arity 1 != 24 |
| time_series_rank | TimeSeriesRank | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| triangle_pattern | TrianglePattern | Wickra | FAIL | **FAIL** (err 1.0e+00, nan 19) | yes | yes | **FAIL** (err 1.0e+00, nan 19) |
| triple_top_bottom | TripleTopBottom | Wickra | FAIL | **FAIL** (err 2.0e+00, nan 5) | yes | yes | **FAIL** (err 2.0e+00, nan 5) |
| true_strength_index | TrueStrengthIndex | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| ulcer_index | UlcerIndex | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| up_down_volume_ratio | UpDownVolumeRatio | Wickra | ERROR | — | — | yes | — |
| value_when | ValueWhen | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| vidya | VariableIndexDynamicAverage | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| volume_by_time_profile | VolumeByTimeProfile | Wickra | FAIL | error: output arity 1 != 24 | yes | yes | error: output arity 1 != 24 |
| volume_price_trend | VolumePriceTrend | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| volume_weighted_moving_average | VolumeWeightedMovingAverage | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| volume_weighted_moving_average_convergence_divergence | VolumeWeightedMovingAverageConvergenceDivergence | Wickra | FAIL | error: output arity 1 != 3 | yes | yes | error: output arity 1 != 3 |
| vortex | Vortex | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| wedge_pattern | WedgePattern | Wickra | FAIL | **FAIL** (err 2.0e+00, nan 19) | yes | yes | **FAIL** (err 2.0e+00, nan 19) |
| yang_zhang | YangZhang | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| zero_lag_exponential_moving_average | ZeroLagExponentialMovingAverage | — | NO_EXTERNAL_ORACLE | — | yes | yes | — |
| zig_zag | ZigZag | Wickra | FAIL | **FAIL** (err 1.1e+02, nan 102) | yes | yes | **FAIL** (err 1.1e+02, nan 102) |
| ACCBANDS | AccelerationBands | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ACOS | MathAcos | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AD | AccumulationDistribution | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ADD | MathAdd | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ADOSC | AccumulationDistributionOscillator | TA-Lib | MATCH | pass (err 5.6e-09, nan 0) | yes | yes | pass (err 5.6e-09, nan 0) |
| ADX | AverageDirectionalIndex | TA-Lib | MATCH | pass (err 1.8e-14, nan 0) | yes | yes | pass (err 1.8e-14, nan 0) |
| ADXR | AverageDirectionalIndexRating | TA-Lib | MATCH | pass (err 2.1e-14, nan 0) | yes | yes | pass (err 2.1e-14, nan 0) |
| APO | AbsolutePriceOscillator | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AROON | Aroon | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AROONOSC | AroonOscillator | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| ASIN | MathAsin | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ATAN | MathAtan | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ATR | AverageTrueRange | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AVGDEV | RollingAverageDeviation | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| AVGPRICE | AveragePrice | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| BBANDS | BollingerBands | TA-Lib | MATCH | pass (err 1.9e-10, nan 0) | yes | yes | pass (err 1.9e-10, nan 0) |
| BETA | RollingBeta | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| BOP | BalanceOfPower | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CCI | CommodityChannelIndex | TA-Lib | MATCH | pass (err 7.7e-12, nan 0) | yes | yes | pass (err 7.7e-12, nan 0) |
| CDL2CROWS | CandleTwoCrows | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3BLACKCROWS | CandleThreeBlackCrows | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3INSIDE | CandleThreeInside | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3LINESTRIKE | CandleThreeLineStrike | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3OUTSIDE | CandleThreeOutside | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3STARSINSOUTH | CandleThreeStarsInSouth | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDL3WHITESOLDIERS | CandleThreeWhiteSoldiers | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLABANDONEDBABY | CandleAbandonedBaby | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLADVANCEBLOCK | CandleAdvanceBlock | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLBELTHOLD | CandleBeltHold | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLBREAKAWAY | CandleBreakaway | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLCLOSINGMARUBOZU | CandleClosingMarubozu | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLCONCEALBABYSWALL | CandleConcealBabySwall | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLCOUNTERATTACK | CandleCounterAttack | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLDARKCLOUDCOVER | CandleDarkCloudCover | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLDOJI | CandleDoji | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLDOJISTAR | CandleDojiStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLDRAGONFLYDOJI | CandleDragonflyDoji | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLENGULFING | CandleEngulfing | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLEVENINGDOJISTAR | CandleEveningDojiStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLEVENINGSTAR | CandleEveningStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLGAPSIDESIDEWHITE | CandleGapSideSideWhite | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLGRAVESTONEDOJI | CandleGravestoneDoji | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHAMMER | CandleHammer | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHANGINGMAN | CandleHangingMan | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHARAMI | CandleHarami | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHARAMICROSS | CandleHaramiCross | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHIGHWAVE | CandleHighWave | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHIKKAKE | CandleHikkake | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHIKKAKEMOD | CandleHikkakeModified | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLHOMINGPIGEON | CandleHomingPigeon | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLIDENTICAL3CROWS | CandleIdenticalThreeCrows | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLINNECK | CandleInNeck | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLINVERTEDHAMMER | CandleInvertedHammer | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLKICKING | CandleKicking | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLKICKINGBYLENGTH | CandleKickingByLength | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLLADDERBOTTOM | CandleLadderBottom | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLLONGLEGGEDDOJI | CandleLongLeggedDoji | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLLONGLINE | CandleLongLine | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMARUBOZU | CandleMarubozu | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMATCHINGLOW | CandleMatchingLow | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMATHOLD | CandleMatHold | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMORNINGDOJISTAR | CandleMorningDojiStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLMORNINGSTAR | CandleMorningStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLONNECK | CandleOnNeck | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLPIERCING | CandlePiercing | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLRICKSHAWMAN | CandleRickshawman | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLRISEFALL3METHODS | CandleRiseFallThreeMethods | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSEPARATINGLINES | CandleSeparatingLines | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSHOOTINGSTAR | CandleShootingStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSHORTLINE | CandleShortLine | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSPINNINGTOP | CandleSpinningTop | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSTALLEDPATTERN | CandleStalledPattern | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLSTICKSANDWICH | CandleStickSandwich | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLTAKURI | CandleTakuri | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLTASUKIGAP | CandleTasukiGap | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLTHRUSTING | CandleThrusting | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLTRISTAR | CandleTriStar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLUNIQUE3RIVER | CandleUniqueThreeRiver | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLUPSIDEGAP2CROWS | CandleUpsideGapTwoCrows | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CDLXSIDEGAP3METHODS | CandleUpDownSideGapThreeMethods | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CEIL | MathCeil | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| CMO | ChandeMomentumOscillator | TA-Lib | MATCH | pass (err 6.8e-14, nan 0) | yes | yes | pass (err 6.8e-14, nan 0) |
| CORREL | RollingCorrelation | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| COS | MathCos | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| COSH | MathCosh | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| DEMA | DoubleExponentialMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| DIV | MathDivide | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| DX | DirectionalMovementIndex | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| EMA | ExponentialMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| EXP | MathExp | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| FLOOR | MathFloor | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_DCPERIOD | HilbertTransformDominantCyclePeriod | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_DCPHASE | HilbertTransformDominantCyclePhase | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_PHASOR | HilbertTransformPhasor | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_SINE | HilbertTransformSineWave | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_TRENDLINE | HilbertTransformTrendline | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| HT_TRENDMODE | HilbertTransformTrendMode | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| IMI | IntradayMomentumIndex | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| KAMA | KaufmanAdaptiveMovingAverage | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| LINEARREG | RollingLinearRegression | TA-Lib | MATCH | pass (err 2.4e-13, nan 0) | yes | yes | pass (err 2.4e-13, nan 0) |
| LINEARREG_ANGLE | RollingLinearRegressionAngle | TA-Lib | MATCH | pass (err 1.6e-12, nan 0) | yes | yes | pass (err 1.6e-12, nan 0) |
| LINEARREG_INTERCEPT | RollingLinearRegressionIntercept | TA-Lib | MATCH | pass (err 2.3e-13, nan 0) | yes | yes | pass (err 2.3e-13, nan 0) |
| LINEARREG_SLOPE | RollingLinearRegressionSlope | TA-Lib | MATCH | pass (err 3.7e-14, nan 0) | yes | yes | pass (err 3.7e-14, nan 0) |
| LN | MathLn | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| LOG10 | MathLog10 | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MA | MovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MACD | MovingAverageConvergenceDivergence | TA-Lib | MATCH | pass (err 2.2e-16, nan 0) | yes | yes | pass (err 2.2e-16, nan 0) |
| MACDEXT | MovingAverageConvergenceDivergenceExtended | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MACDFIX | MovingAverageConvergenceDivergenceFixed | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| MAMA | MesaAdaptiveMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MAVP | VariablePeriodMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MAX | RollingMaximum | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MAXINDEX | RollingMaximumIndex | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MEDPRICE | MedianPrice | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MFI | MoneyFlowIndex | TA-Lib | MATCH | pass (err 2.1e-14, nan 0) | yes | yes | pass (err 2.1e-14, nan 0) |
| MIDPOINT | RollingMidpoint | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MIDPRICE | RollingMidprice | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MIN | RollingMinimum | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MININDEX | RollingMinimumIndex | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MINMAX | RollingMinMax | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MINMAXINDEX | RollingMinMaxIndex | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MINUS_DI | MinusDirectionalIndicator | TA-Lib | MATCH | pass (err 7.1e-15, nan 0) | yes | yes | pass (err 7.1e-15, nan 0) |
| MINUS_DM | MinusDirectionalMovement | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MOM | Momentum | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| MULT | MathMultiply | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| NATR | NormalizedAverageTrueRange | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| OBV | OnBalanceVolume | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| PLUS_DI | PlusDirectionalIndicator | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| PLUS_DM | PlusDirectionalMovement | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| PPO | PercentagePriceOscillator | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ROC | RateOfChange | TA-Lib | MATCH | pass (err 1.2e-14, nan 0) | yes | yes | pass (err 1.2e-14, nan 0) |
| ROCP | RateOfChangePercent | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ROCR | RateOfChangeRatio | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| ROCR100 | RateOfChangeRatioPercent | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| RSI | RelativeStrengthIndex | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SAR | ParabolicSar | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SAREXT | ParabolicSarExtended | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SIN | MathSin | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SINH | MathSinh | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SMA | SimpleMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SQRT | MathSqrt | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| STDDEV | RollingStandardDeviation | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| STOCH | StochasticOscillator | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| STOCHF | FastStochasticOscillator | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| STOCHRSI | StochasticRelativeStrengthIndex | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SUB | MathSubtract | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| SUM | RollingSum | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| T3 | TripleExponentialAverage | TA-Lib | MATCH | pass (err 2.3e-13, nan 0) | yes | yes | pass (err 2.3e-13, nan 0) |
| TAN | MathTan | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| TANH | MathTanh | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| TEMA | TripleExponentialMovingAverage | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| TRANGE | TrueRange | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| TRIMA | TriangularMovingAverage | TA-Lib | MATCH | pass (err 4.7e-13, nan 0) | yes | yes | pass (err 4.7e-13, nan 0) |
| TRIX | TripleExponentialRateOfChange | TA-Lib | MATCH | pass (err 1.1e-14, nan 0) | yes | yes | pass (err 1.1e-14, nan 0) |
| TSF | RollingTimeSeriesForecast | TA-Lib | MATCH | pass (err 2.8e-13, nan 0) | yes | yes | pass (err 2.8e-13, nan 0) |
| TYPPRICE | TypicalPrice | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| ULTOSC | UltimateOscillator | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| VAR | RollingVariance | TA-Lib | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| WCLPRICE | WeightedClose | TA-Lib | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| WILLR | WilliamsPercentR | TA-Lib | MATCH | pass (err 2.8e-14, nan 0) | yes | yes | pass (err 2.8e-14, nan 0) |
| WMA | WeightedMovingAverage | TA-Lib | MATCH | pass (err 6.5e-12, nan 0) | yes | yes | pass (err 6.5e-12, nan 0) |
| average_daily_range | AverageDailyRange | Wickra | MATCH | pass (err 8.9e-15, nan 0) | yes | yes | pass (err 8.9e-15, nan 0) |
| better_volume | BetterVolume | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| center_of_gravity | CenterOfGravity | Wickra | MATCH | pass (err 2.7e-15, nan 0) | yes | yes | pass (err 2.7e-15, nan 0) |
| decycler | Decycler | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| decycler_oscillator | DecyclerOscillator | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| demand_index | DemandIndex | Wickra | MATCH | pass (err 4.5e-12, nan 0) | yes | yes | pass (err 4.5e-12, nan 0) |
| ehlers_stochastic | EhlersStochastic | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| fair_value_gap | FairValueGap | SMC | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| instantaneous_trendline | InstantaneousTrendline | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| intraday_intensity | IntradayIntensity | Wickra | MATCH | pass (err 1.2e-10, nan 0) | yes | yes | pass (err 1.2e-10, nan 0) |
| inverse_fisher_transform | InverseFisherTransform | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| market_facilitation_index | MarketFacilitationIndex | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| math_abs | MathAbs | NumPy | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| math_acosh | MathAcosh | NumPy | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| math_asinh | MathAsinh | NumPy | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| math_atanh | MathAtanh | NumPy | MATCH | pass (err 8.9e-16, nan 0) | yes | yes | pass (err 8.9e-16, nan 0) |
| math_cbrt | MathCbrt | NumPy | MATCH | pass (err 4.4e-16, nan 0) | yes | yes | pass (err 4.4e-16, nan 0) |
| math_cot | MathCot | NumPy | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| math_degrees | MathDegrees | NumPy | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| math_log1p | MathLog1p | NumPy | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| math_radians | MathRadians | NumPy | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| median_channel | MedianChannel | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| overnight_intraday_return | OvernightIntradayReturn | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| quartile_bands | QuartileBands | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rmi | RelativeMomentumIndex | Wickra | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| rolling_average_drawdown | RollingAverageDrawdown | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_beta_neutral_spread | RollingBetaNeutralSpread | Wickra | MATCH | pass (err 2.1e-11, nan 0) | yes | yes | pass (err 2.1e-11, nan 0) |
| rolling_conditional_value_at_risk | RollingConditionalValueAtRisk | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_drawdown_duration | RollingDrawdownDuration | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_gain_loss_ratio | RollingGainLossRatio | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_granger_causality | RollingGrangerCausality | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_kelly_criterion | RollingKellyCriterion | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_kendall_rank_correlation | RollingKendallRankCorrelation | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_maximum_drawdown | RollingMaximumDrawdown | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_median_absolute_deviation | RollingMedianAbsoluteDeviation | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_omega_ratio | RollingOmegaRatio | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_pain_index | RollingPainIndex | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_pairwise_beta | RollingPairwiseBeta | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_profit_factor | RollingProfitFactor | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_spearman_correlation | RollingSpearmanCorrelation | Wickra | MATCH | pass (err 4.4e-16, nan 0) | yes | yes | pass (err 4.4e-16, nan 0) |
| rolling_standard_error | RollingStandardError | Wickra | MATCH | pass (err 2.0e-15, nan 0) | yes | yes | pass (err 2.0e-15, nan 0) |
| rolling_treynor_ratio | RollingTreynorRatio | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_value_at_risk | RollingValueAtRisk | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| rolling_variance_ratio | RollingVarianceRatio | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| roofing_filter | RoofingFilter | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| session_range | SessionRange | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| session_volume_weighted_average_price | SessionVolumeWeightedAveragePrice | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| sessions | Sessions | SMC | MATCH | pass (err 3.8e-06, nan 0) | yes | yes | pass (err 3.8e-06, nan 0) |
| signed_power | SignedPower | NumPy | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| super_smoother | SuperSmoother | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| time_segmented_volume | TimeSegmentedVolume | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| trade_volume_index | TradeVolumeIndex | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| twiggs_money_flow | TwiggsMoneyFlow | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |
| volume_oscillator | VolumeOscillator | Wickra | MATCH | pass (err 2.6e-13, nan 0) | yes | yes | pass (err 2.6e-13, nan 0) |
| volume_relative_strength_index | VolumeRelativeStrengthIndex | Wickra | MATCH | pass (err 1.4e-14, nan 0) | yes | yes | pass (err 1.4e-14, nan 0) |
| volume_zone_oscillator | VolumeZoneOscillator | Wickra | MATCH | pass (err 5.0e-14, nan 0) | yes | yes | pass (err 5.0e-14, nan 0) |
| williams_accumulation_distribution | WilliamsAccumulationDistribution | Wickra | MATCH | pass (err 0.0e+00, nan 0) | yes | yes | pass (err 0.0e+00, nan 0) |

## Follow-ups

- Mismatches: rolling_cointegration, rolling_lead_lag_cross_correlation, volume_weighted_moving_average_convergence_divergence, moving_average_envelope, average_true_range_bands, linear_regression_channel, standard_error_bands, double_bollinger_bands, hurst_channel, adaptive_cycle, hilbert_dominant_cycle, empirical_mode_decomposition, zig_zag, overnight_gap, time_of_day_return_profile, day_of_week_return_profile, intraday_volatility_profile, volume_by_time_profile, cup_and_handle, rectangle_range, flag_pennant, wedge_pattern, triangle_pattern, head_and_shoulders, triple_top_bottom, three_drives, cypher_pattern, shark_pattern, crab_pattern, bat_pattern, butterfly_pattern, gartley_pattern, four_point_harmonic_pattern, fibonacci_time_zones, fibonacci_channel, fibonacci_arcs, fibonacci_fan, fibonacci_confluence, golden_pocket, automatic_fibonacci, fibonacci_projection, fibonacci_extension
- Errors (class/mapping/runtime): absolute_breadth_index, cumulative_volume_index, bullish_percent_index, up_down_volume_ratio, percent_above_moving_average, high_low_index, new_highs_new_lows, breadth_thrust, arms_index, mc_clellan_summation_index, mc_clellan_oscillator
- Compared at TA-Lib defaults only (unmapped params): CDLABANDONEDBABY, CDLDARKCLOUDCOVER, CDLEVENINGDOJISTAR, CDLEVENINGSTAR, CDLMATHOLD, CDLMORNINGDOJISTAR, CDLMORNINGSTAR
