# External correctness oracles

Bars: **2,000** | Matches: **99** | Documented variants: **26** | Failures: **0** | rtol=1e-08, atol=1e-10

Versions: taflow 0.1.2, pandas-ta-classic 0.6.52, polars 1.43.2, smartmoneyconcepts 0.0.27

| Oracle | Function | Output | Verdict | Max error | NaN mismatches | Note |
|---|---|---|---:|---:|---:|---|
| pandas-ta-classic | `know_sure_thing` | `kst` | VARIANT | `1.824e+04` | 0 | taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100 |
| pandas-ta-classic | `know_sure_thing` | `signal` | VARIANT | `1.782e+04` | 8 | taflow follows the bukosabino/ta KST scaling; pandas-ta multiplies by an extra 100 |
| pandas-ta-classic | `mass_index` | `mass` | VARIANT | `4.737e-02` | 0 | taflow follows bukosabino/ta EMA initialization |
| pandas-ta-classic | `negative_volume_index` | `nvi` | VARIANT | `5.278e+02` | 0 | taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC |
| pandas-ta-classic | `positive_volume_index` | `pvi` | VARIANT | `4.481e+02` | 0 | taflow uses the standard multiplicative index; pandas-ta uses a cumulative volume-weighted ROC |
| smartmoneyconcepts | `break_of_structure_change_of_character` | `bos` | VARIANT | `0.000e+00` | 71 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| smartmoneyconcepts | `break_of_structure_change_of_character` | `broken` | VARIANT | `0.000e+00` | 101 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| smartmoneyconcepts | `break_of_structure_change_of_character` | `choch` | VARIANT | `0.000e+00` | 30 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
| smartmoneyconcepts | `break_of_structure_change_of_character` | `level` | VARIANT | `0.000e+00` | 101 | package retrospectively keeps only broken structures at their pivot; taflow emits causal setup and break events and cannot retract history |
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
| Polars | `abs` | `abs` | MATCH | `0.000e+00` | 0 |  |
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
| pandas-ta-classic | `log_return` | `log_return` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `mcginley_dynamic` | `mcginley` | MATCH | `0.000e+00` | 0 |  |
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
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[length=1]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[length=2]` | MATCH | `0.000e+00` | 0 |  |
| pandas-ta-classic | `variable_index_dynamic_average` | `vidya[length=30]` | MATCH | `1.421e-14` | 0 |  |
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
