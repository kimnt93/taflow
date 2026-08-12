# Metrics benchmark

Generated: 2026-08-12

Public end-to-end semantic-factory `compute()` timings for 52 benchmark-eligible metrics; every row passed the external correctness gate first.

Speedup is reference time divided by TAFlow time; values above 1× favor TAFlow.

Reference libraries: [empyrical-reloaded](https://github.com/stefan-jansen/empyrical-reloaded), [QuantStats](https://github.com/ranaroussi/quantstats), [NumPy](https://numpy.org/), [SciPy](https://scipy.org/), [PerformanceAnalytics](https://cran.r-project.org/package=PerformanceAnalytics), [vectorbt](https://vectorbt.dev/), and [Riskfolio-Lib](https://riskfolio-lib.readthedocs.io/).

| **Class** | **Target** | **1k** | **10k** | **100k** |
|---|---|---:|---:|---:|
| Alpha | empyrical-reloaded 0.5.12 | 1.09x | 0.36x | 0.68x |
| AnnualizedReturn | empyrical-reloaded 0.5.12 | 0.82x | 0.19x | 0.12x |
| AnnualizedVolatility | empyrical-reloaded 0.5.12 | 0.96x | 0.38x | 0.29x |
| AverageLoss | quantstats 0.0.81 | 46.64x | 5.88x | 1.64x |
| AverageWin | quantstats 0.0.81 | 43.44x | 5.66x | 1.76x |
| Beta | empyrical-reloaded 0.5.12 | 0.61x | 0.22x | 0.32x |
| BreakevenRate | numpy 2.4.6 | 0.91x | 0.36x | 0.17x |
| CalmarRatio | empyrical-reloaded 0.5.12 | 1.29x | 0.41x | 0.37x |
| CaptureRatio | empyrical-reloaded 0.5.12 | 0.41x | 0.18x | 0.12x |
| CoefficientOfDetermination | quantstats 0.0.81 | 36.90x | 4.40x | 2.01x |
| CommonSenseRatio | quantstats 0.0.81 | 66.96x | 6.20x | 2.45x |
| CompositeProfitabilityConsistencyIndex | quantstats 0.0.81 | 102.04x | 16.36x | 7.77x |
| ConditionalDrawdownAtRisk | numpy 2.4.6 | 23.12x | 19.74x | 18.95x |
| DeflatedSharpeRatio | scipy 1.18.0 | 14.82x | 2.05x | 0.67x |
| DownMarketCaptureRatio | empyrical-reloaded 0.5.12 | 1.00x | 0.45x | 0.38x |
| DownsideDeviation | empyrical-reloaded 0.5.12 | 2.30x | 0.63x | 0.40x |
| EffectiveNumberOfBets | numpy 2.4.6 | 0.81x | 0.38x | 0.32x |
| EntropicValueAtRisk | scipy 1.18.0 | 6.79x | 1.48x | 1.15x |
| Expectancy | quantstats 0.0.81 | 56.76x | 7.32x | 2.18x |
| Exposure | quantstats 0.0.81 | 66.33x | 9.40x | 1.21x |
| GainToPainRatio | quantstats 0.0.81 | 136.82x | 15.03x | 8.95x |
| GrossLoss | numpy 2.4.6 | 0.98x | 0.63x | 0.59x |
| GrossProfit | numpy 2.4.6 | 1.06x | 0.62x | 0.59x |
| HistoricalExpectedShortfall | empyrical-reloaded 0.5.12 | 0.65x | 0.11x | 0.07x |
| HistoricalValueAtRisk | empyrical-reloaded 0.5.12 | 3.03x | 0.32x | 0.33x |
| InformationRatio | empyrical-reloaded 0.5.12 | 1.47x | 0.37x | 0.23x |
| KellyCriterion | quantstats 0.0.81 | 61.08x | 8.55x | 4.54x |
| LongestLosingStreak | quantstats 0.0.81 | 105.80x | 17.81x | 13.66x |
| LongestWinningStreak | quantstats 0.0.81 | 94.67x | 17.20x | 13.79x |
| MaximumDrawdown | empyrical-reloaded 0.5.12 | 2.57x | 1.04x | 1.40x |
| ModifiedSharpeRatio | scipy 1.18.0 | 2.59x | 0.92x | 0.74x |
| NetProfit | numpy 2.4.6 | 0.74x | 0.14x | 0.05x |
| OmegaRatio | empyrical-reloaded 0.5.12 | 7.03x | 7.38x | 7.34x |
| PainIndex | numpy 2.4.6 | 2.72x | 0.96x | 0.76x |
| ParametricExpectedShortfall | scipy 1.18.0 | 18.20x | 2.35x | 0.42x |
| ParametricValueAtRisk | scipy 1.18.0 | 10.95x | 1.54x | 0.32x |
| PayoffRatio | quantstats 0.0.81 | 121.66x | 17.07x | 7.68x |
| ProbabilisticSharpeRatio | scipy 1.18.0 | 13.55x | 1.95x | 0.69x |
| ProfitFactor | quantstats 0.0.81 | 58.10x | 7.29x | 2.62x |
| RecoveryFactor | quantstats 0.0.81 | 92.15x | 13.85x | 7.37x |
| SharpeRatio | empyrical-reloaded 0.5.12 | 2.52x | 0.78x | 0.53x |
| SortinoRatio | empyrical-reloaded 0.5.12 | 3.23x | 0.95x | 0.59x |
| StabilityOfTimeSeries | empyrical-reloaded 0.5.12 | 21.00x | 2.54x | 1.51x |
| TailRatio | empyrical-reloaded 0.5.12 | 6.42x | 0.91x | 0.60x |
| TotalReturn | empyrical-reloaded 0.5.12 | 0.95x | 0.23x | 0.15x |
| TrackingError | numpy 2.4.6 | 0.87x | 0.18x | 0.11x |
| Turnover | numpy 2.4.6 | 2.23x | 0.53x | 0.25x |
| UlcerIndex | quantstats 0.0.81 | 177.97x | 26.08x | 14.21x |
| UlcerPerformanceIndex | quantstats 0.0.81 | 67.30x | 8.68x | 4.94x |
| UpDownCaptureRatio | empyrical-reloaded 0.5.12 | 0.80x | 0.35x | 0.27x |
| UpMarketCaptureRatio | empyrical-reloaded 0.5.12 | 1.08x | 0.46x | 0.41x |
| WinRate | quantstats 0.0.81 | 47.77x | 5.97x | 1.79x |
