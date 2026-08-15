# Metrics benchmark

Generated: 2026-08-15

Public end-to-end instance-input `compute()` timings for 52 benchmark-eligible metrics; every row passed the external correctness gate first.

Speedup is reference time divided by TAFlow time; values above 1× favor TAFlow.

Reference libraries: [empyrical-reloaded](https://github.com/stefan-jansen/empyrical-reloaded), [QuantStats](https://github.com/ranaroussi/quantstats), [NumPy](https://numpy.org/), [SciPy](https://scipy.org/), [PerformanceAnalytics](https://cran.r-project.org/package=PerformanceAnalytics), [vectorbt](https://vectorbt.dev/), and [Riskfolio-Lib](https://riskfolio-lib.readthedocs.io/).

| **Class** | **Target** | **1k** | **10k** | **100k** |
|---|---|---:|---:|---:|
| TotalReturn | empyrical-reloaded 0.5.12 | 1.45x | 0.35x | 0.23x |
| AnnualizedReturn | empyrical-reloaded 0.5.12 | 1.48x | 0.38x | 0.23x |
| AnnualizedVolatility | empyrical-reloaded 0.5.12 | 1.16x | 0.45x | 0.38x |
| MaximumDrawdown | empyrical-reloaded 0.5.12 | 4.28x | 1.81x | 2.58x |
| DownsideDeviation | empyrical-reloaded 0.5.12 | 5.95x | 2.36x | 1.49x |
| SharpeRatio | empyrical-reloaded 0.5.12 | 3.12x | 1.02x | 0.69x |
| SortinoRatio | empyrical-reloaded 0.5.12 | 8.28x | 3.02x | 2.28x |
| CalmarRatio | empyrical-reloaded 0.5.12 | 2.92x | 1.00x | 0.73x |
| OmegaRatio | empyrical-reloaded 0.5.12 | 26.85x | 15.21x | 14.08x |
| HistoricalValueAtRisk | empyrical-reloaded 0.5.12 | 7.52x | 1.47x | 1.52x |
| HistoricalExpectedShortfall | empyrical-reloaded 0.5.12 | 2.17x | 0.72x | 0.49x |
| TailRatio | empyrical-reloaded 0.5.12 | 7.71x | 1.78x | 1.28x |
| TrackingError | numpy 2.4.6 | 1.48x | 0.30x | 0.19x |
| InformationRatio | empyrical-reloaded 0.5.12 | 2.40x | 0.55x | 0.38x |
| Beta | empyrical-reloaded 0.5.12 | 0.81x | 0.30x | 0.45x |
| Alpha | empyrical-reloaded 0.5.12 | 1.39x | 0.44x | 0.86x |
| CoefficientOfDetermination | quantstats 0.0.81 | 49.38x | 6.16x | 5.59x |
| CaptureRatio | empyrical-reloaded 0.5.12 | 0.55x | 0.23x | 0.15x |
| UpMarketCaptureRatio | empyrical-reloaded 0.5.12 | 1.38x | 0.56x | 0.53x |
| DownMarketCaptureRatio | empyrical-reloaded 0.5.12 | 1.28x | 0.58x | 0.48x |
| UpDownCaptureRatio | empyrical-reloaded 0.5.12 | 0.95x | 0.40x | 0.34x |
| UlcerIndex | quantstats 0.0.81 | 164.33x | 24.08x | 13.17x |
| UlcerPerformanceIndex | quantstats 0.0.81 | 59.49x | 8.15x | 4.24x |
| RecoveryFactor | quantstats 0.0.81 | 85.28x | 13.01x | 7.00x |
| GainToPainRatio | quantstats 0.0.81 | 397.81x | 29.20x | 14.45x |
| PainIndex | numpy 2.4.6 | 2.60x | 0.89x | 0.67x |
| StabilityOfTimeSeries | empyrical-reloaded 0.5.12 | 19.03x | 2.35x | 1.32x |
| BreakevenRate | numpy 2.4.6 | 2.67x | 1.56x | 0.76x |
| WinRate | quantstats 0.0.81 | 148.46x | 11.54x | 3.16x |
| AverageWin | quantstats 0.0.81 | 129.28x | 11.89x | 2.91x |
| AverageLoss | quantstats 0.0.81 | 132.16x | 11.69x | 3.12x |
| PayoffRatio | quantstats 0.0.81 | 93.03x | 14.75x | 6.71x |
| ProfitFactor | quantstats 0.0.81 | 175.40x | 15.35x | 5.38x |
| LongestLosingStreak | quantstats 0.0.81 | 76.98x | 14.22x | 11.49x |
| LongestWinningStreak | quantstats 0.0.81 | 76.26x | 14.30x | 11.11x |
| NetProfit | numpy 2.4.6 | 1.12x | 0.20x | 0.07x |
| GrossProfit | numpy 2.4.6 | 3.39x | 1.39x | 1.09x |
| GrossLoss | numpy 2.4.6 | 3.30x | 1.31x | 1.07x |
| Expectancy | quantstats 0.0.81 | 40.17x | 5.79x | 1.72x |
| KellyCriterion | quantstats 0.0.81 | 57.99x | 7.91x | 4.16x |
| CommonSenseRatio | quantstats 0.0.81 | 69.89x | 10.24x | 4.47x |
| CompositeProfitabilityConsistencyIndex | quantstats 0.0.81 | 146.80x | 22.50x | 11.45x |
| ModifiedSharpeRatio | scipy 1.18.0 | 2.41x | 0.83x | 0.66x |
| ProbabilisticSharpeRatio | scipy 1.18.0 | 13.32x | 1.80x | 0.68x |
| DeflatedSharpeRatio | scipy 1.18.0 | 14.58x | 1.92x | 0.69x |
| ParametricValueAtRisk | scipy 1.18.0 | 13.94x | 1.96x | 0.42x |
| ParametricExpectedShortfall | scipy 1.18.0 | 21.82x | 3.01x | 0.51x |
| ConditionalDrawdownAtRisk | numpy 2.4.6 | 23.39x | 17.71x | 17.86x |
| EntropicValueAtRisk | scipy 1.18.0 | 6.93x | 1.48x | 1.16x |
| Exposure | quantstats 0.0.81 | 26.14x | 2.88x | 0.39x |
| EffectiveNumberOfBets | numpy 2.4.6 | 0.84x | 0.37x | 0.30x |
| Turnover | numpy 2.4.6 | 2.28x | 0.56x | 0.32x |

## Metric pipeline amortization

One Rust-owned P&L conversion and fan-out pass is compared with constructing the same eight TAFlow metric classes separately. This is an internal architecture comparison, not an external-oracle claim; results are gated by equality with the standalone public classes.

| **Metrics** | **Input** | **1k** | **10k** | **100k** |
|---|---|---:|---:|---:|
| 8 whole-history metrics | period P&L | 0.94x | 1.00x | 0.99x |

## Implementation interpretation

The public adapter performs one contiguous container conversion and releases the GIL for native bulk work. Rust bulk loops hoist semantic-domain validation and avoid per-observation result calculation. They deliberately retain chronological scalar accumulation for Welford moments, compensated sums, compounding, and drawdown state; these recurrences are not reassociated into SIMD reductions because scalar append, chunked extend, and batch extend must leave the same persistent state. Exact historical tails use cached linear-time selection rather than a full sort.

Consequently, array libraries can remain faster for simple one-shot reductions that use highly tuned vector kernels, while TAFlow's advantages are persistent O(1) continuation, cached O(1) reads, native streaming, and amortizing one semantic conversion across a metric pipeline.

## SharpeRatio execution profiles (100k observations)

These profiles separate native bulk processing from Python scalar/chunk boundary costs and cached reads.

| **Path** | **Median** |
|---|---:|
| native bulk | 0.514 ms |
| chunks 1024 | 0.577 ms |
| chunks 32 | 2.690 ms |
| scalar append | 16.222 ms |
| warmed continuation | 0.292 ms |
| cached compute | 79.2 ns |
