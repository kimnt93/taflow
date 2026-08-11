# Metrics correctness

Generated: 2026-08-11

Every TAFlow value below came from the public canonical class factory and `compute()`.

| Metric | Oracle | Result | Maximum absolute error | Maximum relative error |
|---|---|---:|---:|---:|
| `TotalReturn` | empyrical-reloaded 0.5.12 `cum_returns_final` | **MATCH** | 1.776e-15 | 1.026e-14 |
| `AnnualizedReturn` | empyrical-reloaded 0.5.12 `annual_return` | **MATCH** | 1.115e+25 | 1.557e-13 |
| `AnnualizedVolatility` | empyrical-reloaded 0.5.12 `annual_volatility` | **MATCH** | 3.553e-15 | 3.299e-03 |
| `MaximumDrawdown` | empyrical-reloaded 0.5.12 `max_drawdown` | **MATCH** | 1.110e-16 | 2.812e-15 |
| `DownsideDeviation` | empyrical-reloaded 0.5.12 `downside_risk` | **MATCH** | 0.000e+00 | 0.000e+00 |
| `SharpeRatio` | empyrical-reloaded 0.5.12 `sharpe_ratio` | **MATCH** | 1.294e-05 | 1.281e-03 |
| `SortinoRatio` | empyrical-reloaded 0.5.12 `sortino_ratio` | **MATCH** | 0.000e+00 | 0.000e+00 |
| `CalmarRatio` | empyrical-reloaded 0.5.12 `calmar_ratio` | **MATCH** | 2.811e-08 | 1.123e-13 |
| `OmegaRatio` | empyrical-reloaded 0.5.12 `omega_ratio` | **MATCH** | 1.088e-14 | 9.993e-15 |
| `HistoricalValueAtRisk` | empyrical-reloaded 0.5.12 `value_at_risk` | **MATCH** | 1.735e-18 | 3.469e-16 |
| `HistoricalExpectedShortfall` | empyrical-reloaded 0.5.12 `conditional_value_at_risk` | **MATCH** | 6.939e-18 | 5.713e-16 |
| `TailRatio` | empyrical-reloaded 0.5.12 `tail_ratio` | **MATCH** | 1.110e-16 | 2.416e-16 |
| `TrackingError` | numpy 2.4.6 `std` | **MATCH** | 8.882e-16 | 5.017e-16 |
| `InformationRatio` | empyrical-reloaded 0.5.12 `excess_sharpe` | **MATCH** | 1.137e-13 | 1.619e-13 |
| `Beta` | empyrical-reloaded 0.5.12 `beta_aligned` | **MATCH** | 1.824e-14 | 1.824e-01 |
| `Alpha` | empyrical-reloaded 0.5.12 `alpha_aligned` | **MATCH** | 6.803e-13 | 1.002e-13 |
| `CoefficientOfDetermination` | quantstats 0.0.81 `r_squared` | **MATCH** | 3.616e-13 | 1.641e-11 |
| `CaptureRatio` | empyrical-reloaded 0.5.12 `capture` | **MATCH** | 1.626e+12 | 1.285e-12 |
| `UpMarketCaptureRatio` | empyrical-reloaded 0.5.12 `up_capture` | **MATCH** | 2.975e+134 | 1.063e-12 |
| `DownMarketCaptureRatio` | empyrical-reloaded 0.5.12 `down_capture` | **MATCH** | 4.619e-14 | 1.603e-14 |
| `UpDownCaptureRatio` | empyrical-reloaded 0.5.12 `up_down_capture` | **MATCH** | 2.975e+134 | 1.063e-12 |
| `TreynorRatio` | quantstats 0.0.81 `treynor_ratio` | **VARIANT** | 3.031e-07 | 1.582e-11 |
| `UlcerIndex` | quantstats 0.0.81 `ulcer_index` | **MATCH** | 5.551e-17 | 2.497e-16 |
| `UlcerPerformanceIndex` | quantstats 0.0.81 `ulcer_performance_index` | **MATCH** | 1.554e-14 | 9.973e-15 |
| `RecoveryFactor` | quantstats 0.0.81 `recovery_factor` | **MATCH** | 2.220e-16 | 1.727e-15 |
| `GainToPainRatio` | quantstats 0.0.81 `gain_to_pain_ratio` | **MATCH** | 7.494e-16 | 2.838e-14 |
| `PainIndex` | numpy 2.4.6 `mean` | **VARIANT** | 1.110e-16 | 1.005e-15 |
| `PainRatio` | numpy 2.4.6 `mean` | **VARIANT** | 6.775e-08 | 1.137e-13 |
| `AverageDrawdown` | numpy 2.4.6 `mean` | **VARIANT** | 0.000e+00 | 0.000e+00 |
| `MaximumDrawdownDuration` | numpy 2.4.6 `max` | **VARIANT** | 0.000e+00 | 0.000e+00 |
| `StabilityOfTimeSeries` | empyrical-reloaded 0.5.12 `stability_of_timeseries` | **MATCH** | 1.055e-15 | 2.964e-15 |
| `BreakevenRate` | numpy 2.4.6 `mean` | **MATCH** | 0.000e+00 | 0.000e+00 |
| `WinRate` | quantstats 0.0.81 `win_rate` | **MATCH** | 0.000e+00 | 0.000e+00 |
| `AverageWin` | quantstats 0.0.81 `avg_win` | **MATCH** | 6.939e-18 | 6.987e-16 |
| `AverageLoss` | quantstats 0.0.81 `avg_loss` | **MATCH** | 6.939e-18 | 7.712e-16 |
| `PayoffRatio` | quantstats 0.0.81 `payoff_ratio` | **MATCH** | 6.661e-16 | 6.516e-16 |
| `ProfitFactor` | quantstats 0.0.81 `profit_factor` | **MATCH** | 6.661e-16 | 6.146e-16 |
| `LongestLosingStreak` | quantstats 0.0.81 `consecutive_losses` | **MATCH** | 0.000e+00 | 0.000e+00 |
| `LongestWinningStreak` | quantstats 0.0.81 `consecutive_wins` | **MATCH** | 0.000e+00 | 0.000e+00 |
| `NetProfit` | numpy 2.4.6 `sum` | **MATCH** | 2.776e-17 | 1.864e-15 |
| `GrossProfit` | numpy 2.4.6 `sum` | **MATCH** | 1.776e-15 | 7.015e-16 |
| `GrossLoss` | numpy 2.4.6 `sum` | **MATCH** | 1.332e-15 | 6.094e-16 |
| `Expectancy` | quantstats 0.0.81 `avg_win` | **MATCH** | 3.469e-18 | 6.361e-14 |
| `KellyCriterion` | quantstats 0.0.81 `kelly_criterion` | **MATCH** | 3.553e-15 | 1.108e-13 |
| `SystemQualityNumber` | numpy 2.4.6 `mean` | **VARIANT** | 4.592e-06 | 7.615e-12 |
| `CommonSenseRatio` | quantstats 0.0.81 `common_sense_ratio` | **MATCH** | 1.332e-15 | 1.175e-15 |
| `CompositeProfitabilityConsistencyIndex` | quantstats 0.0.81 `cpc_index` | **MATCH** | 7.772e-16 | 1.363e-15 |

Environment: Python 3.12.3, NumPy 2.4.6.
