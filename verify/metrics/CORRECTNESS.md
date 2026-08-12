# Metrics correctness

Generated: 2026-08-12

Every TAFlow value below came from the public canonical class factory and `compute()`.

`MATCH` means every registered dataset, parameter row, and lifecycle check passed the metric's declared absolute/relative tolerance. The displayed maximum absolute and relative errors may come from different cases.

| Metric | Oracle package | Oracle source function | Result | Maximum absolute error | Maximum relative error |
|---|---|---|---:|---:|---:|
| `TotalReturn` | empyrical-reloaded 0.5.12 | [`empyrical.stats.cum_returns_final`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.776e-15 | 1.026e-14 |
| `AnnualizedReturn` | empyrical-reloaded 0.5.12 | [`empyrical.stats.annual_return`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.115e+25 | 1.557e-13 |
| `AnnualizedVolatility` | empyrical-reloaded 0.5.12 | [`empyrical.stats.annual_volatility`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 3.553e-15 | 3.299e-03 |
| `MaximumDrawdown` | empyrical-reloaded 0.5.12 | [`empyrical.stats.max_drawdown`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.110e-16 | 2.812e-15 |
| `DownsideDeviation` | empyrical-reloaded 0.5.12 | [`empyrical.stats.downside_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `SharpeRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.sharpe_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.294e-05 | 1.281e-03 |
| `SortinoRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.sortino_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `CalmarRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.calmar_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 2.811e-08 | 1.123e-13 |
| `OmegaRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.omega_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.088e-14 | 9.993e-15 |
| `HistoricalValueAtRisk` | empyrical-reloaded 0.5.12 | [`empyrical.stats.value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.735e-18 | 3.469e-16 |
| `HistoricalExpectedShortfall` | empyrical-reloaded 0.5.12 | [`empyrical.stats.conditional_value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 6.939e-18 | 5.713e-16 |
| `TailRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.tail_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.110e-16 | 2.416e-16 |
| `TrackingError` | numpy 2.4.6 | [`numpy.std`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 8.882e-16 | 5.017e-16 |
| `InformationRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.excess_sharpe`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.137e-13 | 1.619e-13 |
| `Beta` | empyrical-reloaded 0.5.12 | [`empyrical.stats.beta_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.824e-14 | 1.824e-01 |
| `Alpha` | empyrical-reloaded 0.5.12 | [`empyrical.stats.alpha_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 6.803e-13 | 1.002e-13 |
| `CoefficientOfDetermination` | quantstats 0.0.81 | [`quantstats.stats.r_squared`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 3.616e-13 | 1.641e-11 |
| `CaptureRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.626e+12 | 1.285e-12 |
| `UpMarketCaptureRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.up_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 2.975e+134 | 1.063e-12 |
| `DownMarketCaptureRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 4.619e-14 | 1.603e-14 |
| `UpDownCaptureRatio` | empyrical-reloaded 0.5.12 | [`empyrical.stats.up_down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 2.975e+134 | 1.063e-12 |
| `TreynorRatio` | PerformanceAnalytics 2.1.0 | [`PerformanceAnalytics::TreynorRatio`](https://cran.r-project.org/src/contrib/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 2.830e-05 | 1.582e-11 |
| `UlcerIndex` | quantstats 0.0.81 | [`quantstats.stats.ulcer_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 5.551e-17 | 2.497e-16 |
| `UlcerPerformanceIndex` | quantstats 0.0.81 | [`quantstats.stats.ulcer_performance_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1.554e-14 | 9.973e-15 |
| `RecoveryFactor` | quantstats 0.0.81 | [`quantstats.stats.recovery_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 2.220e-16 | 1.727e-15 |
| `GainToPainRatio` | quantstats 0.0.81 | [`quantstats.stats.gain_to_pain_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 7.494e-16 | 2.838e-14 |
| `PainIndex` | numpy 2.4.6 | [`PerformanceAnalytics::PainIndex`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 1.110e-16 | 1.005e-15 |
| `PainRatio` | numpy 2.4.6 | [`PerformanceAnalytics::PainRatio`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 6.775e-08 | 1.137e-13 |
| `AverageDrawdown` | numpy 2.4.6 | [`PerformanceAnalytics::AverageDrawdown`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `MaximumDrawdownDuration` | numpy 2.4.6 | [`PerformanceAnalytics::findDrawdowns`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `StabilityOfTimeSeries` | empyrical-reloaded 0.5.12 | [`empyrical.stats.stability_of_timeseries`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1.055e-15 | 2.964e-15 |
| `BreakevenRate` | numpy 2.4.6 | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `WinRate` | quantstats 0.0.81 | [`quantstats.stats.win_rate`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `AverageWin` | quantstats 0.0.81 | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 6.939e-18 | 6.987e-16 |
| `AverageLoss` | quantstats 0.0.81 | [`quantstats.stats.avg_loss`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 6.939e-18 | 7.712e-16 |
| `PayoffRatio` | quantstats 0.0.81 | [`quantstats.stats.payoff_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 6.661e-16 | 6.516e-16 |
| `ProfitFactor` | quantstats 0.0.81 | [`quantstats.stats.profit_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 6.661e-16 | 6.146e-16 |
| `LongestLosingStreak` | quantstats 0.0.81 | [`quantstats.stats.consecutive_losses`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `LongestWinningStreak` | quantstats 0.0.81 | [`quantstats.stats.consecutive_wins`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `NetProfit` | numpy 2.4.6 | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 2.776e-17 | 1.864e-15 |
| `GrossProfit` | numpy 2.4.6 | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1.776e-15 | 7.015e-16 |
| `GrossLoss` | numpy 2.4.6 | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1.332e-15 | 6.094e-16 |
| `Expectancy` | quantstats 0.0.81 | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 3.469e-18 | 6.361e-14 |
| `KellyCriterion` | quantstats 0.0.81 | [`quantstats.stats.kelly_criterion`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 3.553e-15 | 1.108e-13 |
| `SystemQualityNumber` | numpy 2.4.6 | [`vectorbt.Trades.sqn`](https://github.com/polakowo/vectorbt/blob/v0.28.1/vectorbt/portfolio/trades.py) | **MATCH** | 4.592e-06 | 7.615e-12 |
| `CommonSenseRatio` | quantstats 0.0.81 | [`quantstats.stats.common_sense_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1.332e-15 | 1.175e-15 |
| `CompositeProfitabilityConsistencyIndex` | quantstats 0.0.81 | [`quantstats.stats.cpc_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 7.772e-16 | 1.363e-15 |
| `ModifiedSharpeRatio` | scipy 1.18.0 | [`PerformanceAnalytics::SharpeRatio.modified(FUN='VaR')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 1.497e-09 | 2.413e-08 |
| `ProbabilisticSharpeRatio` | scipy 1.18.0 | [`vectorbt probabilistic Sharpe kernel + scipy.stats.norm.cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 8.882e-16 | 3.926e-15 |
| `DeflatedSharpeRatio` | scipy 1.18.0 | [`vectorbt deflated Sharpe kernel + scipy.stats.norm.ppf/cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 3.291e-10 | 1.315e-08 |
| `ParametricValueAtRisk` | scipy 1.18.0 | [`scipy.stats.norm.ppf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 1.055e-09 | 1.178e-09 |
| `ParametricExpectedShortfall` | scipy 1.18.0 | [`scipy.stats.norm.ppf/pdf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 3.580e-09 | 3.126e-09 |
| `ConditionalDrawdownAtRisk` | numpy 2.4.6 | [`PerformanceAnalytics::CDD(method='discrete')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `EntropicValueAtRisk` | scipy 1.18.0 | [`riskfolio.RiskFunctions.EVaR_Hist`](https://github.com/dcajasn/Riskfolio-Lib/blob/632a9e48fbaf2b9f8e83864a492332364b6ed32c/riskfolio/src/RiskFunctions.py) | **MATCH** | 1.110e-16 | 5.677e-16 |
| `Exposure` | quantstats 0.0.81 | [`quantstats.stats.exposure`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 0.000e+00 | 0.000e+00 |
| `EffectiveNumberOfBets` | numpy 2.4.6 | [`numpy.linalg.eigh + numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/linalg/_linalg.py) | **MATCH** | 2.046e-12 | 5.336e-15 |
| `Turnover` | numpy 2.4.6 | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1.388e-17 | 6.429e-16 |

Environment: Python 3.12.3, NumPy 2.4.6.
