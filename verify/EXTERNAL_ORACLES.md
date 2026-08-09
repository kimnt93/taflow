# External correctness oracles

Bars: **2,000** | Matches: **200** | Documented variants: **38** | Failures: **0** | rtol=1e-08, atol=1e-10

Versions: taflow 0.1.2, numpy 2.4.6, pandas-ta-classic 0.6.52, polars 1.43.2, smartmoneyconcepts 0.0.27, wickra 0.9.9

| Oracle | Function | Output | Verdict | Max error | NaN mismatches | Note |
|---|---|---|---:|---:|---:|---|
| pandas-ta-classic | `arnaud_legoux_moving_average` | `alma` | VARIANT | `5.611e+00` | 0 | independently compared; documented initialization/formula convention differs |
| pandas-ta-classic | `chaikin_volatility` | `chaikin_volatility` | VARIANT | `2.563e+00` | 9 | independently compared; documented initialization/formula convention differs |
| pandas-ta-classic | `ease_of_movement` | `ease_of_movement` | VARIANT | `4.531e+03` | 0 | independently compared; documented initialization/formula convention differs |
| pandas-ta-classic | `keltner_channels` | `lower` | VARIANT | `3.751e+00` | 20 | independently compared; documented initialization/formula convention differs |
| pandas-ta-classic | `keltner_channels` | `middle` | VARIANT | `4.587e-01` | 19 | independently compared; documented initialization/formula convention differs |
| pandas-ta-classic | `keltner_channels` | `upper` | VARIANT | `3.846e+00` | 20 | independently compared; documented initialization/formula convention differs |
| pandas-ta-classic | `know_sure_thing` | `kst` | VARIANT | `1.824e+04` | 0 | taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100 |
| pandas-ta-classic | `know_sure_thing` | `signal` | VARIANT | `1.782e+04` | 8 | taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100 |
| pandas-ta-classic | `mass_index` | `mass` | VARIANT | `4.737e-02` | 0 | taflow follows bukosabino/ta EMA initialization |
| pandas-ta-classic | `negative_volume_index` | `nvi` | VARIANT | `5.278e+02` | 0 | taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC |
| pandas-ta-classic | `positive_volume_index` | `pvi` | VARIANT | `4.481e+02` | 0 | taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC |
| pandas-ta-classic | `true_strength_index` | `tsi` | VARIANT | `7.286e+01` | 36 | independently compared; documented initialization/formula convention differs |
| pandas-ta-classic | `ulcer_index` | `ulcer_index` | VARIANT | `1.169e+01` | 13 | independently compared; documented initialization/formula convention differs |
| pandas-ta-classic | `volume_price_trend` | `volume_price_trend` | VARIANT | `6.091e+07` | 0 | independently compared; documented initialization/formula convention differs |
| smartmoneyconcepts | `break_of_structure_change_of_character` | `bos` | VARIANT | `0.000e+00` | 71 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| smartmoneyconcepts | `break_of_structure_change_of_character` | `broken` | VARIANT | `0.000e+00` | 101 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| smartmoneyconcepts | `break_of_structure_change_of_character` | `choch` | VARIANT | `0.000e+00` | 30 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| smartmoneyconcepts | `break_of_structure_change_of_character` | `level` | VARIANT | `0.000e+00` | 101 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| smartmoneyconcepts | `equal_highs_lows` | `equal_high` | VARIANT | `0.000e+00` | 43 | SMC liquidity pools are the external equal-high/low analogue; TAFlow emits causal ATR-thresholded confirmations |
| smartmoneyconcepts | `equal_highs_lows` | `equal_low` | VARIANT | `0.000e+00` | 47 | SMC liquidity pools are the external equal-high/low analogue; TAFlow emits causal ATR-thresholded confirmations |
| smartmoneyconcepts | `equal_highs_lows` | `level` | VARIANT | `0.000e+00` | 51 | SMC liquidity pools are the external equal-high/low analogue; TAFlow emits causal ATR-thresholded confirmations |
| smartmoneyconcepts | `liquidity` | `level` | VARIANT | `1.500e+00` | 184 | package uses full-series range and retroactive group starts; taflow uses causal level-relative tolerance and emits second-touch/sweep events |
| smartmoneyconcepts | `liquidity` | `liquidity` | VARIANT | `2.000e+00` | 125 | package uses full-series range and retroactive group starts; taflow uses causal level-relative tolerance and emits second-touch/sweep events |
| smartmoneyconcepts | `liquidity` | `swept` | VARIANT | `0.000e+00` | 71 | package uses full-series range and retroactive group starts; taflow uses causal level-relative tolerance and emits second-touch/sweep events |
| smartmoneyconcepts | `order_block` | `bottom` | VARIANT | `0.000e+00` | 170 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| smartmoneyconcepts | `order_block` | `mitigated` | VARIANT | `0.000e+00` | 117 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| smartmoneyconcepts | `order_block` | `ob` | VARIANT | `0.000e+00` | 170 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| smartmoneyconcepts | `order_block` | `ob_volume` | VARIANT | `0.000e+00` | 170 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| smartmoneyconcepts | `order_block` | `top` | VARIANT | `0.000e+00` | 170 | different published definitions: package retroactively marks and deletes historical blocks; taflow is causal, dual-pivot, and ATR-filtered |
| smartmoneyconcepts | `previous_high_low` | `broken_high` | VARIANT | `1.000e+00` | 989 | SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day |
| smartmoneyconcepts | `previous_high_low` | `broken_low` | VARIANT | `1.000e+00` | 987 | SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day |
| smartmoneyconcepts | `previous_high_low` | `previous_high` | VARIANT | `4.564e+01` | 96 | SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day |
| smartmoneyconcepts | `previous_high_low` | `previous_low` | VARIANT | `3.688e+01` | 96 | SMC 0.0.27 selects periods_before-2 (two-day lag); taflow exposes the immediately previous day |
| smartmoneyconcepts | `retracements` | `current_retracement_pct` | VARIANT | `6.093e+02` | 60 | package is lookahead-aligned, rounded, and uses candle extremes; taflow confirms swings causally and measures the current close |
| smartmoneyconcepts | `retracements` | `deepest_retracement_pct` | VARIANT | `6.093e+02` | 60 | package is lookahead-aligned, rounded, and uses candle extremes; taflow confirms swings causally and measures the current close |
| smartmoneyconcepts | `retracements` | `direction` | VARIANT | `2.000e+00` | 14 | package is lookahead-aligned, rounded, and uses candle extremes; taflow confirms swings causally and measures the current close |
| smartmoneyconcepts | `swing_highs_lows` | `level` | VARIANT | `0.000e+00` | 41 | package also removes markers retroactively; causal taflow cannot retract emitted events |
| smartmoneyconcepts | `swing_highs_lows` | `signal` | VARIANT | `0.000e+00` | 41 | package also removes markers retroactively; causal taflow cannot retract emitted events |
| NumPy | `math_abs` | `all` | MATCH | `0.000e+00` | 0 | numpy.abs |
| NumPy | `math_acosh` | `all` | MATCH | `0.000e+00` | 0 | numpy.arccosh |
| NumPy | `math_asinh` | `all` | MATCH | `0.000e+00` | 0 | numpy.arcsinh |
| NumPy | `math_atanh` | `all` | MATCH | `1.110e-16` | 0 | numpy.arctanh |
| NumPy | `math_cbrt` | `all` | MATCH | `6.661e-16` | 0 | numpy.cbrt |
| NumPy | `math_cot` | `all` | MATCH | `0.000e+00` | 0 | numpy.tan reciprocal |
| NumPy | `math_degrees` | `all` | MATCH | `0.000e+00` | 0 | numpy.degrees |
| NumPy | `math_log1p` | `all` | MATCH | `0.000e+00` | 0 | numpy.log1p |
| NumPy | `math_radians` | `all` | MATCH | `0.000e+00` | 0 | numpy.radians |
| NumPy | `signed_power` | `all` | MATCH | `0.000e+00` | 0 | numpy.sign/numpy.abs/numpy.power |
| Polars | `ceil` | `ceil` | MATCH | `0.000e+00` | 0 |  |
| Polars | `correlation` | `correlation` | MATCH | `2.254e-11` | 0 |  |
| Polars | `cos` | `cos` | MATCH | `0.000e+00` | 0 |  |
| Polars | `covariance` | `covariance` | MATCH | `5.835e-12` | 0 |  |
| Polars | `cumulative_maximum` | `cumulative_maximum` | MATCH | `0.000e+00` | 0 |  |
| Polars | `cumulative_minimum` | `cumulative_minimum` | MATCH | `0.000e+00` | 0 |  |
| Polars | `cumulative_product` | `cumulative_product` | MATCH | `7.105e-15` | 0 |  |
| Polars | `cumulative_sum` | `cumulative_sum` | MATCH | `0.000e+00` | 0 |  |
| Polars | `ewm_stddev` | `ewm_stddev` | MATCH | `1.776e-15` | 0 |  |
| Polars | `ewm_variance` | `ewm_variance` | MATCH | `1.066e-14` | 0 |  |
| Polars | `exp` | `exp` | MATCH | `4.441e-16` | 0 |  |
| Polars | `floor` | `floor` | MATCH | `0.000e+00` | 0 |  |
| Polars | `kurtosis` | `kurtosis` | MATCH | `4.695e-05` | 0 | Polars raw-moment kernel tolerance after centering/scaling |
| Polars | `ln` | `ln` | MATCH | `0.000e+00` | 0 |  |
| Polars | `math_abs` | `math_abs` | MATCH | `0.000e+00` | 0 |  |
| Polars | `max` | `max` | MATCH | `0.000e+00` | 0 |  |
| Polars | `mean` | `mean` | MATCH | `2.274e-13` | 0 |  |
| Polars | `median` | `median` | MATCH | `0.000e+00` | 0 |  |
| Polars | `min` | `min` | MATCH | `0.000e+00` | 0 |  |
| Polars | `quantile` | `quantile` | MATCH | `0.000e+00` | 0 |  |
| Polars | `sin` | `sin` | MATCH | `0.000e+00` | 0 |  |
| Polars | `skew` | `skew` | MATCH | `8.776e-08` | 0 | Polars raw-moment kernel tolerance after centering/scaling |
| Polars | `sqrt` | `sqrt` | MATCH | `0.000e+00` | 0 |  |
| Polars | `stddev` | `stddev` | MATCH | `2.402e-11` | 0 |  |
| Polars | `sum` | `sum` | MATCH | `3.070e-12` | 0 |  |
| Polars | `tan` | `tan` | MATCH | `0.000e+00` | 0 |  |
| Polars | `variance` | `variance` | MATCH | `3.930e-11` | 0 |  |
| Wickra | `relative_momentum_index` | `period=1,momentum=1` | MATCH | `1.421e-14` | 0 | wickra.RMI Wilder-seeded state; version 0.9.9 |
| Wickra | `relative_momentum_index` | `period=14,momentum=5` | MATCH | `1.421e-14` | 0 | wickra.RMI Wilder-seeded state; version 0.9.9 |
| Wickra | `relative_momentum_index` | `period=3,momentum=2` | MATCH | `1.421e-14` | 0 | wickra.RMI Wilder-seeded state; version 0.9.9 |
| Wickra | `relative_momentum_index` | `period=30,momentum=12` | MATCH | `1.421e-14` | 0 | wickra.RMI Wilder-seeded state; version 0.9.9 |
| pandas | `amihud` | `all` | MATCH | `2.895e-24` | 0 | Series.pct_change/rolling.mean |
| pandas | `anchored_vwap` | `lower` | MATCH | `1.062e-11` | 0 | pandas grouped cumulative weighted moments |
| pandas | `anchored_vwap` | `upper` | MATCH | `1.062e-11` | 0 | pandas grouped cumulative weighted moments |
| pandas | `anchored_vwap` | `vwap` | MATCH | `8.527e-14` | 0 | pandas grouped cumulative weighted moments |
| pandas | `average_daily_dollar_value` | `all` | MATCH | `9.686e-08` | 0 | Series.rolling.mean |
| pandas | `bars_since` | `all` | MATCH | `0.000e+00` | 0 | Series.groupby.cumcount |
| pandas | `close_to_close_sigma` | `all` | MATCH | `4.857e-17` | 0 | Series.rolling.std(log returns) |
| pandas | `cumulative_count` | `all` | MATCH | `0.000e+00` | 0 | Series.size/arange |
| pandas | `cumulative_sum_control_chart` | `all` | MATCH | `0.000e+00` | 0 | NumPy implementation of AFML CUSUM |
| pandas | `decay_linear` | `all` | MATCH | `2.103e-12` | 0 | Series.rolling.apply/numpy.dot |
| pandas | `drawdown` | `all` | MATCH | `0.000e+00` | 0 | Series.cummax |
| pandas | `ewm_corr` | `all` | MATCH | `1.265e-14` | 0 | ExponentialMovingWindow.corr |
| pandas | `ewm_cov` | `all` | MATCH | `2.558e-13` | 0 | ExponentialMovingWindow.cov(bias=True) |
| pandas | `ewm_sum` | `all` | MATCH | `0.000e+00` | 0 | ExponentialMovingWindow.sum |
| pandas | `falling` | `all` | MATCH | `0.000e+00` | 0 | Series.shift comparison |
| pandas | `fibonacci_retracement` | `0` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `fibonacci_retracement` | `0.236` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `fibonacci_retracement` | `0.382` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `fibonacci_retracement` | `0.5` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `fibonacci_retracement` | `0.618` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `fibonacci_retracement` | `0.786` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `fibonacci_retracement` | `1` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `frac_diff` | `all` | MATCH | `1.155e-13` | 0 | NumPy AFML fixed-width fractional differentiation |
| pandas | `fractal_dimension` | `all` | MATCH | `1.399e-14` | 0 | two minus pandas rescaled-range Hurst |
| pandas | `gap_down` | `all` | MATCH | `0.000e+00` | 0 | Series.shift comparison |
| pandas | `gap_up` | `all` | MATCH | `0.000e+00` | 0 | Series.shift comparison |
| pandas | `garman_klass` | `all` | MATCH | `1.995e-17` | 0 | Garman-Klass via Rolling.mean |
| pandas | `garman_klass_yang_zhang` | `all` | MATCH | `2.429e-17` | 0 | Garman-Klass-Yang-Zhang via Rolling.mean |
| pandas | `hedge_ratio` | `all` | MATCH | `1.353e-09` | 0 | Rolling.cov/Rolling.var(ddof=0) |
| pandas | `higher_high` | `all` | MATCH | `0.000e+00` | 0 | Series.shift comparison |
| pandas | `highest_since` | `all` | MATCH | `0.000e+00` | 0 | Series.groupby.cummax |
| pandas | `hurst` | `all` | MATCH | `1.388e-14` | 0 | pandas Rolling.apply rescaled-range estimator |
| pandas | `inside_bar` | `all` | MATCH | `0.000e+00` | 0 | Series.shift comparison |
| pandas | `kalman_hedge_ratio` | `beta` | MATCH | `0.000e+00` | 0 | NumPy two-state Kalman filter_update |
| pandas | `lag` | `all` | MATCH | `0.000e+00` | 0 | Series.shift |
| pandas | `lower_low` | `all` | MATCH | `0.000e+00` | 0 | Series.shift comparison |
| pandas | `lowest_since` | `all` | MATCH | `0.000e+00` | 0 | Series.groupby.cummin |
| pandas | `opening_range` | `breakout` | MATCH | `0.000e+00` | 0 | NumPy anchored opening-range definition |
| pandas | `opening_range` | `high` | MATCH | `0.000e+00` | 0 | NumPy anchored opening-range definition |
| pandas | `opening_range` | `low` | MATCH | `0.000e+00` | 0 | NumPy anchored opening-range definition |
| pandas | `ornstein_uhlenbeck_half_life` | `all` | MATCH | `3.547e-09` | 0 | Rolling.cov/Rolling.var OU regression |
| pandas | `outside_bar` | `all` | MATCH | `0.000e+00` | 0 | Series.shift comparison |
| pandas | `parkinson` | `all` | MATCH | `2.602e-17` | 0 | Parkinson estimator via Rolling.mean |
| pandas | `pivot_points` | `pivot` | MATCH | `0.000e+00` | 0 | NumPy anchored OHLC pivot definition |
| pandas | `pivot_points` | `r1` | MATCH | `0.000e+00` | 0 | NumPy anchored OHLC pivot definition |
| pandas | `pivot_points` | `r2` | MATCH | `0.000e+00` | 0 | NumPy anchored OHLC pivot definition |
| pandas | `pivot_points` | `s1` | MATCH | `0.000e+00` | 0 | NumPy anchored OHLC pivot definition |
| pandas | `pivot_points` | `s2` | MATCH | `0.000e+00` | 0 | NumPy anchored OHLC pivot definition |
| pandas | `premium_discount` | `equilibrium` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `premium_discount` | `zone` | MATCH | `0.000e+00` | 0 | Rolling.min/Rolling.max |
| pandas | `rising` | `all` | MATCH | `0.000e+00` | 0 | Series.shift comparison |
| pandas | `rogers_satchell` | `all` | MATCH | `1.821e-17` | 0 | Rogers-Satchell via Rolling.mean |
| pandas | `roll_spread` | `all` | MATCH | `1.623e-14` | 0 | Series.diff/Rolling.cov(ddof=1) |
| pandas | `rolling_alpha` | `all` | MATCH | `8.285e-10` | 0 | Rolling.cov/Rolling.var/Rolling.mean |
| pandas | `rolling_autocorr` | `all` | MATCH | `6.661e-16` | 0 | Series.rolling.apply/Series.corr |
| pandas | `rolling_calmar` | `all` | MATCH | `1.819e-12` | 0 | Series.rolling.apply |
| pandas | `rolling_entropy` | `all` | MATCH | `4.441e-16` | 0 | Series.rolling.apply/value_counts |
| pandas | `rolling_information_ratio` | `all` | MATCH | `2.886e-11` | 0 | Rolling.mean/Rolling.std(ddof=0) |
| pandas | `rolling_mode` | `all` | MATCH | `0.000e+00` | 0 | Series.rolling.apply/value_counts |
| pandas | `rolling_rank` | `all` | MATCH | `0.000e+00` | 0 | Series.rolling.apply |
| pandas | `rolling_sharpe` | `all` | MATCH | `6.300e-09` | 0 | Rolling.mean/Rolling.std(ddof=0) |
| pandas | `rolling_sortino` | `all` | MATCH | `0.000e+00` | 0 | Series.rolling.apply |
| pandas | `rolling_vwap` | `all` | MATCH | `5.684e-14` | 0 | Series.rolling.sum |
| pandas | `rolling_winsorize` | `all` | MATCH | `1.421e-14` | 0 | Series.rolling.quantile/numpy.clip |
| pandas | `session_volume_levels` | `poc` | MATCH | `0.000e+00` | 0 | NumPy fixed-bin anchored volume profile |
| pandas | `session_volume_levels` | `vah` | MATCH | `0.000e+00` | 0 | NumPy fixed-bin anchored volume profile |
| pandas | `session_volume_levels` | `val` | MATCH | `0.000e+00` | 0 | NumPy fixed-bin anchored volume profile |
| pandas | `signal_delay` | `all` | MATCH | `0.000e+00` | 0 | Series.shift |
| pandas | `spread_zscore` | `all` | MATCH | `4.680e-14` | 0 | numpy rolling OLS/z-score |
| pandas | `ssl_channel` | `lower` | MATCH | `1.137e-13` | 0 | Rolling.mean SSL recurrence |
| pandas | `ssl_channel` | `upper` | MATCH | `1.279e-13` | 0 | Rolling.mean SSL recurrence |
| pandas | `time_series_rank` | `all` | MATCH | `0.000e+00` | 0 | Series.rolling.apply |
| pandas | `value_when` | `all` | MATCH | `0.000e+00` | 0 | Series.where/ffill |
| pandas | `yang_zhang` | `all` | MATCH | `2.776e-17` | 0 | Yang-Zhang via Rolling.mean |
| pandas-ta-classic | `awesome_oscillator` | `ao` | MATCH | `8.527e-14` | 0 |  |
| pandas-ta-classic | `chaikin_money_flow` | `cmf` | MATCH | `3.993e-14` | 0 |  |
| pandas-ta-classic | `crossover` | `crossover` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `crossunder` | `crossunder` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `detrended_price_oscillator` | `dpo` | MATCH | `1.137e-13` | 0 |  |
| pandas-ta-classic | `donchian_channels` | `lower` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `donchian_channels` | `mid` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `donchian_channels` | `upper` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `even_better_sinewave` | `ebsw` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `even_better_sinewave` | `ebsw[length=60]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `fisher_transform` | `fisher` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `fisher_transform` | `fisher[length=11]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `fisher_transform` | `fisher[length=21]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `force_index` | `force_index` | MATCH | `0.000e+00` | 0 | taflow exposes the unsmoothed one-bar force; pandas-ta EFI length=1 |
| pandas-ta-classic | `heikin_ashi` | `close` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `heikin_ashi` | `high` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `heikin_ashi` | `low` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `heikin_ashi` | `open` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `hull_moving_average` | `hma` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `ichimoku` | `chikou` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| pandas-ta-classic | `ichimoku` | `kijun` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| pandas-ta-classic | `ichimoku` | `span_a` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| pandas-ta-classic | `ichimoku` | `span_b` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| pandas-ta-classic | `ichimoku` | `tenkan` | MATCH | `0.000e+00` | 0 | causal alignment; plotting displacement removed |
| pandas-ta-classic | `jurik_moving_average` | `jma` | MATCH | `2.842e-14` | 0 |  |
| pandas-ta-classic | `jurik_moving_average` | `jma[length=1,phase=0]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `jurik_moving_average` | `jma[length=2,phase=-100]` | MATCH | `1.421e-14` | 0 |  |
| pandas-ta-classic | `jurik_moving_average` | `jma[length=21,phase=35]` | MATCH | `2.842e-14` | 0 |  |
| pandas-ta-classic | `jurik_moving_average` | `jma[length=7,phase=100]` | MATCH | `4.263e-14` | 0 |  |
| pandas-ta-classic | `klinger_volume_oscillator` | `kvo` | MATCH | `2.183e-10` | 0 |  |
| pandas-ta-classic | `klinger_volume_oscillator` | `kvo[fast=5,slow=8,signal=3]` | MATCH | `2.910e-10` | 0 |  |
| pandas-ta-classic | `klinger_volume_oscillator` | `signal` | MATCH | `1.914e-10` | 0 |  |
| pandas-ta-classic | `klinger_volume_oscillator` | `signal[fast=5,slow=8,signal=3]` | MATCH | `2.146e-10` | 0 |  |
| pandas-ta-classic | `laguerre_relative_strength_index` | `lrsi` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `laguerre_relative_strength_index` | `lrsi[constant]` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| pandas-ta-classic | `laguerre_relative_strength_index` | `lrsi[gamma=0.1]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `laguerre_relative_strength_index` | `lrsi[gamma=0.25]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `laguerre_relative_strength_index` | `lrsi[gamma=0.9]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `laguerre_relative_strength_index` | `lrsi[minimum]` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| pandas-ta-classic | `laguerre_relative_strength_index` | `lrsi[monotonic]` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| pandas-ta-classic | `laguerre_relative_strength_index` | `lrsi[repeated]` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| pandas-ta-classic | `log_return` | `log_return` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `mcginley_dynamic` | `mcginley` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `parabolic_moving_average_stop` | `stop` | MATCH | `5.684e-14` | 0 |  |
| pandas-ta-classic | `schaff_trend_cycle` | `macd` | MATCH | `7.105e-14` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| pandas-ta-classic | `schaff_trend_cycle` | `stc` | MATCH | `1.000e-08` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| pandas-ta-classic | `schaff_trend_cycle` | `stochastic` | MATCH | `0.000e+00` | 0 | stream-safe epsilon convention; documented tolerance 1e-5 |
| pandas-ta-classic | `squeeze` | `momentum` | MATCH | `3.553e-15` | 0 |  |
| pandas-ta-classic | `squeeze` | `no` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `squeeze` | `off` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `squeeze` | `on` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `squeeze_pro` | `momentum` | MATCH | `3.553e-15` | 0 |  |
| pandas-ta-classic | `squeeze_pro` | `no` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `squeeze_pro` | `off` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `squeeze_pro` | `on_narrow` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `squeeze_pro` | `on_normal` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `squeeze_pro` | `on_wide` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `supertrend` | `direction` | MATCH | `0.000e+00` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| pandas-ta-classic | `supertrend` | `long` | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| pandas-ta-classic | `supertrend` | `short` | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| pandas-ta-classic | `supertrend` | `trend` | MATCH | `1.421e-14` | 0 | pandas-ta seeds pre-ATR rows; compare from length-1 |
| pandas-ta-classic | `tom_de_mark_sequential` | `buy` | MATCH | `0.000e+00` | 0 | pandas-ta columns reordered and capped at the DeMark setup count of nine |
| pandas-ta-classic | `tom_de_mark_sequential` | `sell` | MATCH | `0.000e+00` | 0 | pandas-ta columns reordered and capped at the DeMark setup count of nine |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[constant]` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[length=1]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[length=2]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[length=30]` | MATCH | `1.421e-14` | 0 |  |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[minimum]` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[monotonic]` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[repeated]` | MATCH | `0.000e+00` | 0 | required source-shape matrix |
| pandas-ta-classic | `volume_weighted_moving_average` | `vwma` | MATCH | `5.684e-14` | 0 |  |
| pandas-ta-classic | `vortex` | `minus` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `vortex` | `plus` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `zero_lag_exponential_moving_average` | `zlema` | MATCH | `7.105e-14` | 0 | force pandas-ta's native EMA; TA-Lib rejects leading ZLMA NaNs |
| pandas-ta-classic | `zero_lag_exponential_moving_average` | `zlema[length=1]` | MATCH | `0.000e+00` | 0 | parameter matrix; force pandas-ta's native EMA |
| pandas-ta-classic | `zero_lag_exponential_moving_average` | `zlema[length=21]` | MATCH | `7.105e-14` | 0 | parameter matrix; force pandas-ta's native EMA |
| pandas-ta-classic | `zero_lag_exponential_moving_average` | `zlema[length=2]` | MATCH | `1.421e-14` | 0 | parameter matrix; force pandas-ta's native EMA |
| smartmoneyconcepts | `fair_value_gap` | `bottom` | MATCH | `0.000e+00` | 0 | package marker at middle candle; shifted to causal detection bar |
| smartmoneyconcepts | `fair_value_gap` | `fvg` | MATCH | `0.000e+00` | 0 | package marker at middle candle; shifted to causal detection bar |
| smartmoneyconcepts | `fair_value_gap` | `mitigated` | MATCH | `0.000e+00` | 0 | package future index converted to flag at mitigation bar |
| smartmoneyconcepts | `fair_value_gap` | `top` | MATCH | `0.000e+00` | 0 | package marker at middle candle; shifted to causal detection bar |
| smartmoneyconcepts | `sessions` | `active` | MATCH | `0.000e+00` | 0 | all-day custom session; boundaries passed as flags |
| smartmoneyconcepts | `sessions` | `high` | MATCH | `3.589e-06` | 0 | all-day custom session; boundaries passed as flags |
| smartmoneyconcepts | `sessions` | `low` | MATCH | `3.808e-06` | 0 | all-day custom session; boundaries passed as flags |
