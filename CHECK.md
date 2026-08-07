# CHECK.md — AI implementation review gates

This file is the review contract for taflow. An implementing agent works
through the checklists in `plans/`; a reviewing AI (or the implementer,
before checking any box) walks EVERY gate below for each function. The
checklists in `plans/` are updated continuously — always re-read them and
this file before reviewing. A checklist box may only be `[x]` when every
gate here passes.

---

## 1. Module contract

Two public surfaces, ONE implementation:

- **`taflow`** — our style. Descriptive snake_case names, persistent state
  objects with continue-update compute:
  `append(...) / extend(...) / value / compute() / reset()`.
- **`taflow.talib`** — TA-Lib-compatible surface. Uppercase TA-Lib names
  (`BBANDS`, `MIN`, `MAX`, …), TA-Lib parameter names, order, defaults, and
  warm-up/NaN semantics. Thin aliases only.

Rules:

- A numerical kernel exists exactly once, in Rust
  (`crates/taflow-core`). Both surfaces bind the same kernel — never a
  second implementation, never Python math.
- Functions with no TA-Lib counterpart (SMC, rolling_median, …) exist only
  in `taflow` and never get a fake uppercase alias.
- Canonical interface names are descriptive words, not TA-Lib acronyms. For
  example, `ChaikinMoneyFlow`/`chaikin_money_flow` is canonical while `MFI`
  remains only in `taflow.talib`.
- Every Python stateful indicator constructor accepts its complete input
  series signature (for example `high`, `low`, `close`, and `volume`) in
  addition to configuration parameters. Passing those series at construction
  time is equivalent to calling `extend`; later updates use `append` or
  `extend` with the same names and order.
- Public Python and Rust names match their canonical descriptive spelling;
  candle-pattern interfaces use `Candle...`/`candle_...`. Each public
  function has a multi-line docstring or Rust documentation comment that
  describes inputs, parameters, and return values.
- Python must remain an adapter only: every numerical operation behind a
  public indicator must be implemented by a Rust kernel in
  `crates/taflow-core` and exposed through the native binding. Pure-Python
  numerical implementations are review failures until ported.

## 2. Grouping and rename map

### Groups

| Group | Contents | taflow naming |
|---|---|---|
| `rolling` | rolling-window math/statistics (from TA-Lib Math Operators + Statistic families and the operator checklist) | **mandatory `rolling_` prefix** |
| `ma` | moving averages: sma, ema, wma, dema, tema, trima, kama, t3, mama, hull_moving_average, zero_lag_exponential_moving_average, arnaud_legoux_moving_average, volume_weighted_moving_average, jma, vidya, mcginley | indicator names (trader vocabulary), lowercase |
| `momentum` | rsi, macd, stoch, adx, cci, willr, true_strength_index, fisher, … | lowercase indicator names |
| `volatility` | atr, natr, true_range, keltner, donchian, ulcer_index, chaikin_volatility + `rv` estimators (parkinson, garman_klass, rogers_satchell, yang_zhang) | lowercase |
| `volume` | ad, adosc, obv, chaikin_money_flow, force_index, eom, kvo, volume_price_trend, negative_volume_index, positive_volume_index, rolling_vwap | lowercase |
| `price` | avg_price, median_price, typical_price, weighted_close | lowercase descriptive |
| `math` | pointwise transforms (acos … tanh) — NOT rolling, no prefix | lowercase |
| `pattern` | 61 candle patterns | `candle_` prefix: `candle_doji`, `candle_engulfing`, … |
| `cycle` | Hilbert family | `ht_` prefix |
| `smc` | fair_value_gap, swing_highs_lows, bos_choch, order_block, liquidity, … | per recommend checklist P1 |
| `session` | anchored_vwap, pivot_points, opening_range, session volume levels | lowercase |
| `quant` | kalman_hedge_ratio, ornstein_uhlenbeck_half_life, spread_zscore, cusum, frac_diff, amihud, roll_spread | lowercase |
| `ops` | ts_rank, signedpower, lag, crossover helpers | lowercase |

### Rolling rename rule (the important one)

Rolling-window operators are implemented ONCE and exposed as
`taflow.rolling_*`; the un-prefixed name does NOT exist in `taflow` (it
would shadow Python builtins like `min`/`max`/`sum` anyway). The TA-Lib
uppercase name lives only in `taflow.talib`, bound to the same kernel:

| taflow (canonical) | taflow.talib alias |
|---|---|
| `rolling_min` | `MIN` |
| `rolling_max` | `MAX` |
| `rolling_sum` | `SUM` |
| `rolling_argmin` | `MININDEX` |
| `rolling_argmax` | `MAXINDEX` |
| `rolling_minmax` | `MINMAX` |
| `rolling_minmax_index` | `MINMAXINDEX` |
| `rolling_midpoint` | `MIDPOINT` |
| `rolling_std` | `STDDEV` |
| `rolling_var` | `VAR` |
| `rolling_avgdev` | `AVGDEV` |
| `rolling_corr` | `CORREL` |
| `rolling_beta` | `BETA` |
| `rolling_linreg` (+ `_slope`, `_intercept`, `_angle`, `rolling_tsf`) | `LINEARREG` (+ variants, `TSF`) |
| `rolling_median`, `rolling_quantile`, `rolling_rank`, `rolling_zscore`, `rolling_skew`, `rolling_kurtosis`, `rolling_cov`, … | — (no TA-Lib counterpart) |

Moving averages keep their indicator names (`sma`, not `rolling_mean`) —
they are named indicators, and `SMA`/`EMA`/… remain the talib aliases.

### Duplicate consolidations (found reviewing the plans checklists)

- `swing_high` / `swing_low` (operator checklist, pattern section) and the
  P0 causal swing state (recommend checklist) are the SAME function —
  implement once in `smc`/P0, re-export under `ops` naming if wanted.
- Rolling `VWAP` (operator checklist, volume) vs anchored/session VWAP
  (recommend P4) are two distinct functions in one family: `rolling_vwap`
  and `anchored_vwap`. Never a bare `vwap` (ambiguous).
- Extended MAs are one group: HMA/VWMA/ZLEMA/ALMA (operator checklist) +
  JMA/VIDYA/McGinley (recommend P2) all live in `ma`.
- `hedge_ratio` (rolling OLS, operator checklist) vs `kalman_hedge_ratio`
  (recommend P7) are different estimators — both in `quant`, distinct
  names, docstrings cross-referencing each other.
- Price-transform descriptive names (`typical_price` ↔ `TYPPRICE`, etc.)
  follow the same one-kernel/two-surfaces rule as rolling ops.

## 2.5 Master function table (live status)

One row per function: canonical Rust/Python name and the `taflow.talib`
alias (`_` = no TA-Lib counterpart — taflow-only). `[x]` means both the
batch alias and the persistent state exist in the current build; it does
NOT yet certify the §3 gates. The implementing agent updates this table
as functions land; regenerate statuses any time by running the checks in
§5.

### TA-Lib inventory

| Done | implement function rs | implement function py | implement talib |
|---|---|---|---|
| [x] | acceleration_bands | acceleration_bands | ACCBANDS |
| [x] | bollinger_bands | bollinger_bands | BBANDS |
| [x] | double_exponential_moving_average | double_exponential_moving_average | DEMA |
| [x] | exponential_moving_average | exponential_moving_average | EMA |
| [x] | hilbert_transform_trendline | hilbert_transform_trendline | HT_TRENDLINE |
| [x] | kaufman_adaptive_moving_average | kaufman_adaptive_moving_average | KAMA |
| [x] | moving_average | moving_average | MA |
| [x] | mesa_adaptive_moving_average | mesa_adaptive_moving_average | MAMA |
| [x] | variable_period_moving_average | variable_period_moving_average | MAVP |
| [x] | rolling_midpoint | rolling_midpoint | MIDPOINT |
| [x] | rolling_midprice | rolling_midprice | MIDPRICE |
| [x] | parabolic_sar | parabolic_sar | SAR |
| [x] | parabolic_sar_extended | parabolic_sar_extended | SAREXT |
| [x] | simple_moving_average | simple_moving_average | SMA |
| [x] | triple_exponential_average | triple_exponential_average | T3 |
| [x] | triple_exponential_moving_average | triple_exponential_moving_average | TEMA |
| [x] | triangular_moving_average | triangular_moving_average | TRIMA |
| [x] | weighted_moving_average | weighted_moving_average | WMA |
| [x] | average_true_range | average_true_range | ATR |
| [x] | normalized_average_true_range | normalized_average_true_range | NATR |
| [x] | true_range | true_range | TRANGE |
| [x] | rolling_avgdev | rolling_avgdev | AVGDEV |
| [x] | rolling_beta | rolling_beta | BETA |
| [x] | rolling_corr | rolling_corr | CORREL |
| [x] | rolling_linreg | rolling_linreg | LINEARREG |
| [x] | rolling_linreg_angle | rolling_linreg_angle | LINEARREG_ANGLE |
| [x] | rolling_linreg_intercept | rolling_linreg_intercept | LINEARREG_INTERCEPT |
| [x] | rolling_linreg_slope | rolling_linreg_slope | LINEARREG_SLOPE |
| [x] | rolling_std | rolling_std | STDDEV |
| [x] | rolling_tsf | rolling_tsf | TSF |
| [x] | rolling_var | rolling_var | VAR |
| [x] | average_directional_index | average_directional_index | ADX |
| [x] | average_directional_index_rating | average_directional_index_rating | ADXR |
| [x] | absolute_price_oscillator | absolute_price_oscillator | APO |
| [x] | aroon | aroon | AROON |
| [x] | aroon_oscillator | aroon_oscillator | AROONOSC |
| [x] | balance_of_power | balance_of_power | BOP |
| [x] | commodity_channel_index | commodity_channel_index | CCI |
| [x] | chande_momentum_oscillator | chande_momentum_oscillator | CMO |
| [x] | directional_movement_index | directional_movement_index | DX |
| [x] | intraday_momentum_index | intraday_momentum_index | IMI |
| [x] | moving_average_convergence_divergence | moving_average_convergence_divergence | MACD |
| [x] | moving_average_convergence_divergence_extended | moving_average_convergence_divergence_extended | MACDEXT |
| [x] | moving_average_convergence_divergence_fixed | moving_average_convergence_divergence_fixed | MACDFIX |
| [x] | money_flow_index | money_flow_index | MFI |
| [x] | minus_directional_indicator | minus_directional_indicator | MINUS_DI |
| [x] | minus_directional_movement | minus_directional_movement | MINUS_DM |
| [x] | momentum | momentum | MOM |
| [x] | plus_directional_indicator | plus_directional_indicator | PLUS_DI |
| [x] | plus_directional_movement | plus_directional_movement | PLUS_DM |
| [x] | percentage_price_oscillator | percentage_price_oscillator | PPO |
| [x] | rate_of_change | rate_of_change | ROC |
| [x] | rate_of_change_percent | rate_of_change_percent | ROCP |
| [x] | rate_of_change_ratio | rate_of_change_ratio | ROCR |
| [x] | rate_of_change_ratio_percent | rate_of_change_ratio_percent | ROCR100 |
| [x] | relative_strength_index | relative_strength_index | RSI |
| [x] | stochastic_oscillator | stochastic_oscillator | STOCH |
| [x] | fast_stochastic_oscillator | fast_stochastic_oscillator | STOCHF |
| [x] | stochastic_relative_strength_index | stochastic_relative_strength_index | STOCHRSI |
| [x] | triple_exponential_rate_of_change | triple_exponential_rate_of_change | TRIX |
| [x] | ultimate_oscillator | ultimate_oscillator | ULTOSC |
| [x] | williams_r | williams_r | WILLR |
| [x] | add | add | ADD |
| [x] | div | div | DIV |
| [x] | rolling_max | rolling_max | MAX |
| [x] | rolling_argmax | rolling_argmax | MAXINDEX |
| [x] | rolling_min | rolling_min | MIN |
| [x] | rolling_argmin | rolling_argmin | MININDEX |
| [x] | rolling_minmax | rolling_minmax | MINMAX |
| [x] | rolling_minmax_index | rolling_minmax_index | MINMAXINDEX |
| [x] | mult | mult | MULT |
| [x] | sub | sub | SUB |
| [x] | rolling_sum | rolling_sum | SUM |
| [x] | avg_price | avg_price | AVGPRICE |
| [x] | median_price | median_price | MEDPRICE |
| [x] | typical_price | typical_price | TYPPRICE |
| [x] | weighted_close | weighted_close | WCLPRICE |
| [x] | hilbert_transform_dominant_cycle_period | hilbert_transform_dominant_cycle_period | HT_DCPERIOD |
| [x] | hilbert_transform_dominant_cycle_phase | hilbert_transform_dominant_cycle_phase | HT_DCPHASE |
| [x] | hilbert_transform_phasor | hilbert_transform_phasor | HT_PHASOR |
| [x] | hilbert_transform_sine_wave | hilbert_transform_sine_wave | HT_SINE |
| [x] | hilbert_transform_trend_mode | hilbert_transform_trend_mode | HT_TRENDMODE |
| [x] | candle_two_crows | candle_two_crows | CDL2CROWS |
| [x] | candle_three_black_crows | candle_three_black_crows | CDL3BLACKCROWS |
| [x] | candle_three_inside | candle_three_inside | CDL3INSIDE |
| [x] | candle_three_line_strike | candle_three_line_strike | CDL3LINESTRIKE |
| [x] | candle_three_outside | candle_three_outside | CDL3OUTSIDE |
| [x] | candle_three_stars_in_south | candle_three_stars_in_south | CDL3STARSINSOUTH |
| [x] | candle_three_white_soldiers | candle_three_white_soldiers | CDL3WHITESOLDIERS |
| [x] | candle_abandonedbaby | candle_abandonedbaby | CDLABANDONEDBABY |
| [x] | candle_advanceblock | candle_advanceblock | CDLADVANCEBLOCK |
| [x] | candle_belthold | candle_belthold | CDLBELTHOLD |
| [x] | candle_breakaway | candle_breakaway | CDLBREAKAWAY |
| [x] | candle_closingmarubozu | candle_closingmarubozu | CDLCLOSINGMARUBOZU |
| [x] | candle_concealbabyswall | candle_concealbabyswall | CDLCONCEALBABYSWALL |
| [x] | candle_counterattack | candle_counterattack | CDLCOUNTERATTACK |
| [x] | candle_darkcloudcover | candle_darkcloudcover | CDLDARKCLOUDCOVER |
| [x] | candle_doji | candle_doji | CDLDOJI |
| [x] | candle_dojistar | candle_dojistar | CDLDOJISTAR |
| [x] | candle_dragonflydoji | candle_dragonflydoji | CDLDRAGONFLYDOJI |
| [x] | candle_engulfing | candle_engulfing | CDLENGULFING |
| [x] | candle_eveningdojistar | candle_eveningdojistar | CDLEVENINGDOJISTAR |
| [x] | candle_eveningstar | candle_eveningstar | CDLEVENINGSTAR |
| [x] | candle_gapsidesidewhite | candle_gapsidesidewhite | CDLGAPSIDESIDEWHITE |
| [x] | candle_gravestonedoji | candle_gravestonedoji | CDLGRAVESTONEDOJI |
| [x] | candle_hammer | candle_hammer | CDLHAMMER |
| [x] | candle_hangingman | candle_hangingman | CDLHANGINGMAN |
| [x] | candle_harami | candle_harami | CDLHARAMI |
| [x] | candle_haramicross | candle_haramicross | CDLHARAMICROSS |
| [x] | candle_highwave | candle_highwave | CDLHIGHWAVE |
| [x] | candle_hikkake | candle_hikkake | CDLHIKKAKE |
| [x] | candle_hikkake_modified | candle_hikkake_modified | CDLHIKKAKEMOD |
| [x] | candle_homingpigeon | candle_homingpigeon | CDLHOMINGPIGEON |
| [x] | candle_identical_three_crows | candle_identical_three_crows | CDLIDENTICAL3CROWS |
| [x] | candle_inneck | candle_inneck | CDLINNECK |
| [x] | candle_invertedhammer | candle_invertedhammer | CDLINVERTEDHAMMER |
| [x] | candle_kicking | candle_kicking | CDLKICKING |
| [x] | candle_kickingbylength | candle_kickingbylength | CDLKICKINGBYLENGTH |
| [x] | candle_ladderbottom | candle_ladderbottom | CDLLADDERBOTTOM |
| [x] | candle_longleggeddoji | candle_longleggeddoji | CDLLONGLEGGEDDOJI |
| [x] | candle_longline | candle_longline | CDLLONGLINE |
| [x] | candle_marubozu | candle_marubozu | CDLMARUBOZU |
| [x] | candle_matchinglow | candle_matchinglow | CDLMATCHINGLOW |
| [x] | candle_mathold | candle_mathold | CDLMATHOLD |
| [x] | candle_morningdojistar | candle_morningdojistar | CDLMORNINGDOJISTAR |
| [x] | candle_morningstar | candle_morningstar | CDLMORNINGSTAR |
| [x] | candle_onneck | candle_onneck | CDLONNECK |
| [x] | candle_piercing | candle_piercing | CDLPIERCING |
| [x] | candle_rickshawman | candle_rickshawman | CDLRICKSHAWMAN |
| [x] | candle_rise_fall_three_methods | candle_rise_fall_three_methods | CDLRISEFALL3METHODS |
| [x] | candle_separatinglines | candle_separatinglines | CDLSEPARATINGLINES |
| [x] | candle_shootingstar | candle_shootingstar | CDLSHOOTINGSTAR |
| [x] | candle_shortline | candle_shortline | CDLSHORTLINE |
| [x] | candle_spinningtop | candle_spinningtop | CDLSPINNINGTOP |
| [x] | candle_stalledpattern | candle_stalledpattern | CDLSTALLEDPATTERN |
| [x] | candle_sticksandwich | candle_sticksandwich | CDLSTICKSANDWICH |
| [x] | candle_takuri | candle_takuri | CDLTAKURI |
| [x] | candle_tasukigap | candle_tasukigap | CDLTASUKIGAP |
| [x] | candle_thrusting | candle_thrusting | CDLTHRUSTING |
| [x] | candle_tristar | candle_tristar | CDLTRISTAR |
| [x] | candle_unique_three_river | candle_unique_three_river | CDLUNIQUE3RIVER |
| [x] | candle_upside_gap_two_crows | candle_upside_gap_two_crows | CDLUPSIDEGAP2CROWS |
| [x] | candle_up_down_side_gap_three_methods | candle_up_down_side_gap_three_methods | CDLXSIDEGAP3METHODS |
| [x] | accumulation_distribution | accumulation_distribution | AD |
| [x] | accumulation_distribution_oscillator | accumulation_distribution_oscillator | ADOSC |
| [x] | on_balance_volume | on_balance_volume | OBV |
| [x] | acos | acos | ACOS |
| [x] | asin | asin | ASIN |
| [x] | atan | atan | ATAN |
| [x] | ceil | ceil | CEIL |
| [x] | cos | cos | COS |
| [x] | cosh | cosh | COSH |
| [x] | exp | exp | EXP |
| [x] | floor | floor | FLOOR |
| [x] | ln | ln | LN |
| [x] | log10 | log10 | LOG10 |
| [x] | sin | sin | SIN |
| [x] | sinh | sinh | SINH |
| [x] | sqrt | sqrt | SQRT |
| [x] | tan | tan | TAN |
| [x] | tanh | tanh | TANH |

### taflow-only inventory (no TA-Lib counterpart)

| Done | implement function rs | implement function py | implement talib |
|---|---|---|---|
| [x] | lag | lag | _ |
| [x] | log_return | log_return | _ |
| [x] | cumsum | cumsum | _ |
| [x] | cumprod | cumprod | _ |
| [x] | rolling_median | rolling_median | _ |
| [x] | rolling_mode | rolling_mode | _ |
| [x] | rolling_quantile | rolling_quantile | _ |
| [x] | rolling_rank | rolling_rank | _ |
| [x] | rolling_zscore | rolling_zscore | _ |
| [x] | rolling_skew | rolling_skew | _ |
| [x] | rolling_kurtosis | rolling_kurtosis | _ |
| [x] | rolling_cov | rolling_cov | _ |
| [x] | rolling_winsorize | rolling_winsorize | _ |
| [x] | ewm_var | ewm_var | _ |
| [x] | ewm_std | ewm_std | _ |
| [x] | ewm_cov | ewm_cov | _ |
| [x] | ewm_corr | ewm_corr | _ |
| [x] | drawdown | drawdown | _ |
| [x] | cummax | cummax | _ |
| [x] | cummin | cummin | _ |
| [x] | rolling_sharpe | rolling_sharpe | _ |
| [x] | rolling_sortino | rolling_sortino | _ |
| [x] | rolling_calmar | rolling_calmar | _ |
| [x] | hull_moving_average | hull_moving_average | _ |
| [x] | volume_weighted_moving_average | volume_weighted_moving_average | _ |
| [x] | zero_lag_exponential_moving_average | zero_lag_exponential_moving_average | _ |
| [x] | arnaud_legoux_moving_average | arnaud_legoux_moving_average | _ |
| [x] | true_strength_index | true_strength_index | _ |
| [x] | awesome_oscillator | awesome_oscillator | _ |
| [x] | fisher_transform | fisher_transform | _ |
| [x] | keltner_channels | keltner_channels | _ |
| [x] | donchian_channels | donchian_channels | _ |
| [x] | chaikin_volatility | chaikin_volatility | _ |
| [x] | ulcer_index | ulcer_index | _ |
| [x] | rolling_vwap | rolling_vwap | _ |
| [x] | force_index | force_index | _ |
| [x] | ease_of_movement | ease_of_movement | _ |
| [x] | crossover | crossover | _ |
| [x] | crossunder | crossunder | _ |
| [x] | rising | rising | _ |
| [x] | falling | falling | _ |
| [x] | higher_high | higher_high | _ |
| [x] | lower_low | lower_low | _ |
| [x] | inside_bar | inside_bar | _ |
| [x] | outside_bar | outside_bar | _ |
| [x] | gap_up | gap_up | _ |
| [x] | gap_down | gap_down | _ |
| [x] | bars_since | bars_since | _ |
| [x] | value_when | value_when | _ |
| [x] | highest_since | highest_since | _ |
| [x] | lowest_since | lowest_since | _ |
| [x] | signal_delay | signal_delay | _ |
| [x] | swing_highs_lows | swing_highs_lows | _ |
| [x] | hedge_ratio | hedge_ratio | _ |
| [x] | rolling_entropy | rolling_entropy | _ |
| [x] | rolling_autocorr | rolling_autocorr | _ |
| [x] | hurst | hurst | _ |
| [x] | fractal_dimension | fractal_dimension | _ |
| [x] | rolling_alpha | rolling_alpha | _ |
| [x] | rolling_information_ratio | rolling_information_ratio | _ |
| [x] | fair_value_gap | fair_value_gap | _ |
| [x] | bos_choch | bos_choch | _ |
| [x] | order_block | order_block | _ |
| [x] | liquidity | liquidity | _ |
| [x] | equal_highs_lows | equal_highs_lows | _ |
| [x] | previous_high_low | previous_high_low | _ |
| [x] | sessions | sessions | _ |
| [x] | retracements | retracements | _ |
| [x] | premium_discount | premium_discount | _ |
| [x] | supertrend | supertrend | _ |
| [x] | ichimoku | ichimoku | _ |
| [x] | squeeze | squeeze | _ |
| [x] | squeeze_pro | squeeze_pro | _ |
| [x] | schaff_trend_cycle | schaff_trend_cycle | _ |
| [x] | vortex | vortex | _ |
| [x] | know_sure_thing | know_sure_thing | _ |
| [x] | mass_index | mass_index | _ |
| [x] | detrended_price_oscillator | detrended_price_oscillator | _ |
| [x] | chaikin_money_flow | chaikin_money_flow | _ |
| [x] | kvo | kvo | _ |
| [x] | volume_price_trend | volume_price_trend | _ |
| [x] | negative_volume_index | negative_volume_index | _ |
| [x] | positive_volume_index | positive_volume_index | _ |
| [x] | mcginley | mcginley_dynamic | _ |
| [x] | vidya | vidya | _ |
| [x] | laguerre_rsi | laguerre_rsi | _ |
| [x] | rmi | rmi | _ |
| [x] | jma | jma | _ |
| [x] | ssl_channel | ssl_channel | _ |
| [x] | pmax | pmax | _ |
| [x] | td_sequential | td_sequential | _ |
| [x] | even_better_sinewave | even_better_sinewave | _ |
| [x] | fib_retracement | fib_retracement | _ |
| [x] | heikin_ashi | heikin_ashi | _ |
| [x] | anchored_vwap | anchored_vwap | _ |
| [x] | pivot_points | pivot_points | _ |
| [x] | opening_range | opening_range | _ |
| [x] | session_volume_levels | session_volume_levels | _ |
| [x] | parkinson | parkinson | _ |
| [x] | garman_klass | garman_klass | _ |
| [x] | rogers_satchell | rogers_satchell | _ |
| [x] | gk_yang_zhang | gk_yang_zhang | _ |
| [x] | yang_zhang | yang_zhang | _ |
| [x] | close_to_close_sigma | close_to_close_sigma | _ |
| [x] | ts_rank | ts_rank | _ |
| [x] | signedpower | signedpower | _ |
| [x] | average_daily_dollar_value | average_daily_dollar_value | _ |
| [x] | kalman_hedge_ratio | kalman_hedge_ratio | _ |
| [x] | ornstein_uhlenbeck_half_life | ornstein_uhlenbeck_half_life | _ |
| [x] | spread_zscore | spread_zscore | _ |
| [x] | cusum | cusum | _ |
| [x] | frac_diff | frac_diff | _ |
| [x] | amihud | amihud | _ |
| [x] | roll_spread | roll_spread | _ |

## 3. Per-function review checklist

For every function claimed done, verify ALL of:

### 3.1 Implemented enough?

- [ ] Rust persistent state (`append` O(1), `reset`, `value`)
- [ ] Optimized bulk path (`extend_slice`-style; not per-bar `append` loop)
- [ ] Python class in `taflow` with `append/extend/value/compute/reset`
- [ ] Canonical taflow name per the map above; talib alias if applicable
- [ ] Lifecycle tests (construct, append, extend, compute, reset-replay)
- [ ] Oracle parity test (TA-Lib or the pinned reference impl from the
      recommend checklist); self-oracle (state-vs-batch + chunk
      invariance) where no reference exists
- [ ] Appears in `benches/bench.py` output (auto-discovered; run it) with a
      `reports/<FN>.md` + `.json`
- [ ] Same-size causal contract: N bars in → N values out, NaN warm-up,
      no lookahead, chunk-invariant

### 3.2 Optimized? (see `plans/optimize-methods.md`)

- [ ] No per-bar heap allocation; fixed ring buffers, not VecDeque scans
- [ ] Rolling extrema via monotonic deque (single-sided where only one
      side is consumed — §3.2); never O(period) rescans (§3.3)
- [ ] Recurrences use `mul_add`; bulk loops split warm-up prologue from
      branch-free steady state (§4.5); EMA chains fused in bulk (§4.3)
- [ ] Long-stream drift considered for `sum += new - old` accumulators
      (periodic reseed policy, §6.2)
- [ ] Bench evidence: S2 append latency flat in base size; bulk ops/s not
      below TA-Lib without a noted reason

### 3.3 API and typing

- [ ] **No parameter shadows a Python builtin or keyword.** Prefix with
      underscore: `_input`, `_open`, `_min`, `_max`, `_filter`. Applies to
      Python signatures and `#[pyo3(signature = ...)]`. (`high`, `low`,
      `close`, `volume` are fine — they shadow nothing.)
- [ ] Full static type hints on every public Python signature (inputs
      `ArrayLike`/`np.ndarray`, scalars typed, return types explicit)
- [ ] Selector parameters use enums, not magic ints: `MaType` is an
      `enum.IntEnum` (exported from `taflow`), mirrored by a Rust `MaType`
      enum. `taflow.talib` still accepts raw ints for TA-Lib compat and
      converts at the boundary.

### 3.4 Docs and style

- [ ] Rust: `///` doc comments on every public item — one-line summary,
      formula/definition, warm-up rule, complexity, reference matched
- [ ] Python: docstring on every public class/function (NumPy style) with
      the same content; module docstring per file
- [ ] Multi-line formatting only — `rustfmt` and `black`-clean; never
      `def f(self,x):self._s.append(x);return self` one-liners
- [ ] **One function, one file**: Rust
      `crates/taflow-core/src/stream/<name>.rs` with a single public
      state; `mod.rs` contains only `mod`/`pub use` lines. Python
      `python/taflow/<name>.py`, one indicator per file.

## 4. Known debt (found in review, 2026-08-07 — fix as encountered)

1. TA-Lib compat gaps found by the benchmark: 7 patterns reject the
   `penetration` kwarg (CDLABANDONEDBABY, CDLDARKCLOUDCOVER,
   CDLEVENINGDOJISTAR, CDLEVENINGSTAR, CDLMATHOLD, CDLMORNINGDOJISTAR,
   CDLMORNINGSTAR).
2. Pattern logic disagrees with TA-Lib on real data for: CDL3LINESTRIKE,
   CDLADVANCEBLOCK, CDLGAPSIDESIDEWHITE, CDLHIKKAKEMOD, CDLLADDERBOTTOM,
   CDLTRISTAR, CDLUNIQUE3RIVER — diff against TA-Lib C source.
3. Statistics drift at 100k bars (STDDEV/VAR/CORREL/LINEARREG_SLOPE/
   LINEARREG_ANGLE ~1e-9..3e-7; CCI state-path only) — needs the parity
   contract decision + accumulator reseeding (optimize-methods §6.1/§6.2).

## 5. How to run the review

```bash
# build current code
source .venv/bin/activate && maturin develop --release -m crates/taflow-python/Cargo.toml
# correctness + performance evidence for changed functions
python benches/bench.py <FN...> --quick
# style
cargo fmt --check && black --check python/
```

Then walk §3 for each function touched; update the relevant checklist in
`plans/` and this file's §4 as debt is fixed or found.
