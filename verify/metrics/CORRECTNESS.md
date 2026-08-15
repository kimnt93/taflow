# Metrics correctness

Generated: 2026-08-15

Every TAFlow value below came from a configured canonical instance, its input method, and `compute()`.

`MATCH` means every registered dataset, parameter row, and lifecycle check passed the metric's declared absolute/relative tolerance.

Reference libraries and source functions are linked through each Target entry.

| **Class** | **Target** | **Verdict** | **Batch vs oracle** | **Continue vs oracle** |
|---|---|---|---|---|
| TotalReturn | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.8e-15, nan 0) | pass (err 1.8e-15, nan 0) |
| AnnualizedReturn | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.1e+25, nan 0) | pass (err 1.1e+25, nan 0) |
| AnnualizedVolatility | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 3.6e-15, nan 0) | pass (err 3.6e-15, nan 0) |
| MaximumDrawdown | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.1e-16, nan 0) | pass (err 1.1e-16, nan 0) |
| DownsideDeviation | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| SharpeRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.3e-05, nan 0) | pass (err 1.3e-05, nan 0) |
| SortinoRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| CalmarRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 2.8e-08, nan 0) | pass (err 2.8e-08, nan 0) |
| OmegaRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.1e-14, nan 0) | pass (err 1.1e-14, nan 0) |
| HistoricalValueAtRisk | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.7e-18, nan 0) | pass (err 1.7e-18, nan 0) |
| HistoricalExpectedShortfall | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.0e-17, nan 0) | pass (err 1.0e-17, nan 0) |
| TailRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.1e-16, nan 0) | pass (err 1.1e-16, nan 0) |
| TrackingError | [numpy 2.4.6](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | MATCH | pass (err 8.9e-16, nan 0) | pass (err 8.9e-16, nan 0) |
| InformationRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.1e-13, nan 0) | pass (err 1.1e-13, nan 0) |
| Beta | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.8e-14, nan 0) | pass (err 1.8e-14, nan 0) |
| Alpha | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 6.8e-13, nan 0) | pass (err 6.8e-13, nan 0) |
| CoefficientOfDetermination | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 3.6e-13, nan 0) | pass (err 3.6e-13, nan 0) |
| CaptureRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.6e+12, nan 0) | pass (err 1.6e+12, nan 0) |
| UpMarketCaptureRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 3.0e+134, nan 0) | pass (err 3.0e+134, nan 0) |
| DownMarketCaptureRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 4.6e-14, nan 0) | pass (err 4.6e-14, nan 0) |
| UpDownCaptureRatio | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 3.0e+134, nan 0) | pass (err 3.0e+134, nan 0) |
| TreynorRatio | [PerformanceAnalytics 2.1.0](https://cran.r-project.org/src/contrib/PerformanceAnalytics_2.1.0.tar.gz) | MATCH | pass (err 2.8e-05, nan 0) | pass (err 2.8e-05, nan 0) |
| UlcerIndex | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 5.6e-17, nan 0) | pass (err 5.6e-17, nan 0) |
| UlcerPerformanceIndex | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 1.6e-14, nan 0) | pass (err 1.6e-14, nan 0) |
| RecoveryFactor | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 2.2e-16, nan 0) | pass (err 2.2e-16, nan 0) |
| GainToPainRatio | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 7.5e-16, nan 0) | pass (err 7.5e-16, nan 0) |
| PainIndex | [numpy 2.4.6](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | MATCH | pass (err 1.1e-16, nan 0) | pass (err 1.1e-16, nan 0) |
| PainRatio | [numpy 2.4.6](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | MATCH | pass (err 6.8e-08, nan 0) | pass (err 6.8e-08, nan 0) |
| AverageDrawdown | [numpy 2.4.6](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| MaximumDrawdownDuration | [numpy 2.4.6](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| StabilityOfTimeSeries | [empyrical-reloaded 0.5.12](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | MATCH | pass (err 1.1e-15, nan 0) | pass (err 1.1e-15, nan 0) |
| BreakevenRate | [numpy 2.4.6](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| WinRate | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| AverageWin | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 6.9e-18, nan 0) | pass (err 6.9e-18, nan 0) |
| AverageLoss | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 6.9e-18, nan 0) | pass (err 6.9e-18, nan 0) |
| PayoffRatio | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 6.7e-16, nan 0) | pass (err 6.7e-16, nan 0) |
| ProfitFactor | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 6.7e-16, nan 0) | pass (err 6.7e-16, nan 0) |
| LongestLosingStreak | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| LongestWinningStreak | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| NetProfit | [numpy 2.4.6](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | MATCH | pass (err 2.8e-17, nan 0) | pass (err 2.8e-17, nan 0) |
| GrossProfit | [numpy 2.4.6](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | MATCH | pass (err 1.8e-15, nan 0) | pass (err 1.8e-15, nan 0) |
| GrossLoss | [numpy 2.4.6](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | MATCH | pass (err 1.3e-15, nan 0) | pass (err 1.3e-15, nan 0) |
| Expectancy | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 3.5e-18, nan 0) | pass (err 3.5e-18, nan 0) |
| KellyCriterion | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 3.6e-15, nan 0) | pass (err 3.6e-15, nan 0) |
| SystemQualityNumber | [numpy 2.4.6](https://github.com/polakowo/vectorbt/blob/v0.28.1/vectorbt/portfolio/trades.py) | MATCH | pass (err 4.6e-06, nan 0) | pass (err 4.6e-06, nan 0) |
| CommonSenseRatio | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 1.3e-15, nan 0) | pass (err 1.3e-15, nan 0) |
| CompositeProfitabilityConsistencyIndex | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 7.8e-16, nan 0) | pass (err 7.8e-16, nan 0) |
| ModifiedSharpeRatio | [scipy 1.18.0](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | MATCH | pass (err 1.5e-09, nan 0) | pass (err 1.5e-09, nan 0) |
| ProbabilisticSharpeRatio | [scipy 1.18.0](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | MATCH | pass (err 8.9e-16, nan 0) | pass (err 8.9e-16, nan 0) |
| DeflatedSharpeRatio | [scipy 1.18.0](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | MATCH | pass (err 3.3e-10, nan 0) | pass (err 3.3e-10, nan 0) |
| ParametricValueAtRisk | [scipy 1.18.0](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | MATCH | pass (err 1.1e-09, nan 0) | pass (err 1.1e-09, nan 0) |
| ParametricExpectedShortfall | [scipy 1.18.0](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | MATCH | pass (err 3.6e-09, nan 0) | pass (err 3.6e-09, nan 0) |
| ConditionalDrawdownAtRisk | [numpy 2.4.6](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| EntropicValueAtRisk | [scipy 1.18.0](https://github.com/dcajasn/Riskfolio-Lib/blob/632a9e48fbaf2b9f8e83864a492332364b6ed32c/riskfolio/src/RiskFunctions.py) | MATCH | pass (err 1.1e-16, nan 0) | pass (err 1.1e-16, nan 0) |
| Exposure | [quantstats 0.0.81](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | MATCH | pass (err 0.0e+00, nan 0) | pass (err 0.0e+00, nan 0) |
| EffectiveNumberOfBets | [numpy 2.4.6](https://github.com/numpy/numpy/blob/v2.4.6/numpy/linalg/_linalg.py) | MATCH | pass (err 2.0e-12, nan 0) | pass (err 2.0e-12, nan 0) |
| Turnover | [numpy 2.4.6](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | MATCH | pass (err 1.4e-17, nan 0) | pass (err 1.4e-17, nan 0) |

Environment: Python 3.12.3, NumPy 2.4.6.
