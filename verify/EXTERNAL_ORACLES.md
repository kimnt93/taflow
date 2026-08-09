# External correctness oracles

Bars: **2,000** | Matches: **78** | Documented variants: **14** | Failures: **0** | rtol=1e-08, atol=1e-10

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
