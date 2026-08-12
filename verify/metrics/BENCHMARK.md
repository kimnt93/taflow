# Metrics benchmark

Generated: 2026-08-12

Public end-to-end semantic-factory `compute()` timings for 52 benchmark-eligible metrics; every row passed the external correctness gate first.

Speedup is reference time divided by TAFlow time; values above 1× favor TAFlow.

Reference libraries: [empyrical-reloaded](https://github.com/stefan-jansen/empyrical-reloaded), [QuantStats](https://github.com/ranaroussi/quantstats), [NumPy](https://numpy.org/), [SciPy](https://scipy.org/), [PerformanceAnalytics](https://cran.r-project.org/package=PerformanceAnalytics), [vectorbt](https://vectorbt.dev/), and [Riskfolio-Lib](https://riskfolio-lib.readthedocs.io/).

| **Class** | **Target** | **1k** | **10k** | **100k** |
|---|---|---:|---:|---:|
| TotalReturn | empyrical-reloaded 0.5.12 | 1.51x | 0.34x | 0.22x |
| AnnualizedReturn | empyrical-reloaded 0.5.12 | 1.28x | 0.36x | 0.22x |
| AnnualizedVolatility | empyrical-reloaded 0.5.12 | 1.20x | 0.47x | 0.37x |
| MaximumDrawdown | empyrical-reloaded 0.5.12 | 3.94x | 1.70x | 2.25x |
| DownsideDeviation | empyrical-reloaded 0.5.12 | 4.92x | 1.93x | 1.25x |
| SharpeRatio | empyrical-reloaded 0.5.12 | 3.03x | 0.94x | 0.68x |
| SortinoRatio | empyrical-reloaded 0.5.12 | 6.65x | 2.54x | 1.77x |
| CalmarRatio | empyrical-reloaded 0.5.12 | 2.92x | 0.96x | 0.69x |
| OmegaRatio | empyrical-reloaded 0.5.12 | 22.80x | 14.02x | 11.42x |
| HistoricalValueAtRisk | empyrical-reloaded 0.5.12 | 7.72x | 1.46x | 1.52x |
| HistoricalExpectedShortfall | empyrical-reloaded 0.5.12 | 2.20x | 0.77x | 0.50x |
| TailRatio | empyrical-reloaded 0.5.12 | 7.60x | 1.81x | 1.27x |
| TrackingError | numpy 2.4.6 | 1.30x | 0.29x | 0.14x |
| InformationRatio | empyrical-reloaded 0.5.12 | 2.24x | 0.58x | 0.34x |
| Beta | empyrical-reloaded 0.5.12 | 0.80x | 0.30x | 0.45x |
| Alpha | empyrical-reloaded 0.5.12 | 1.54x | 0.47x | 0.81x |
| CoefficientOfDetermination | quantstats 0.0.81 | 47.02x | 5.77x | 2.82x |
| CaptureRatio | empyrical-reloaded 0.5.12 | 0.53x | 0.24x | 0.16x |
| UpMarketCaptureRatio | empyrical-reloaded 0.5.12 | 1.40x | 0.59x | 0.54x |
| DownMarketCaptureRatio | empyrical-reloaded 0.5.12 | 1.27x | 0.60x | 0.51x |
| UpDownCaptureRatio | empyrical-reloaded 0.5.12 | 0.87x | 0.38x | 0.29x |
| UlcerIndex | quantstats 0.0.81 | 161.96x | 23.29x | 12.56x |
| UlcerPerformanceIndex | quantstats 0.0.81 | 57.67x | 8.30x | 4.53x |
| RecoveryFactor | quantstats 0.0.81 | 85.83x | 13.18x | 6.68x |
| GainToPainRatio | quantstats 0.0.81 | 346.49x | 28.83x | 13.40x |
| PainIndex | numpy 2.4.6 | 2.48x | 0.86x | 0.70x |
| StabilityOfTimeSeries | empyrical-reloaded 0.5.12 | 18.29x | 2.33x | 4.74x |
| BreakevenRate | numpy 2.4.6 | 1.92x | 1.23x | 0.60x |
| WinRate | quantstats 0.0.81 | 132.73x | 10.81x | 2.69x |
| AverageWin | quantstats 0.0.81 | 123.24x | 11.00x | 2.74x |
| AverageLoss | quantstats 0.0.81 | 125.42x | 10.70x | 2.70x |
| PayoffRatio | quantstats 0.0.81 | 92.86x | 13.73x | 6.24x |
| ProfitFactor | quantstats 0.0.81 | 159.83x | 15.26x | 5.38x |
| LongestLosingStreak | quantstats 0.0.81 | 73.69x | 13.95x | 10.74x |
| LongestWinningStreak | quantstats 0.0.81 | 69.34x | 13.14x | 10.23x |
| NetProfit | numpy 2.4.6 | 0.99x | 0.19x | 0.07x |
| GrossProfit | numpy 2.4.6 | 2.72x | 1.11x | 1.03x |
| GrossLoss | numpy 2.4.6 | 2.64x | 1.26x | 0.97x |
| Expectancy | quantstats 0.0.81 | 40.75x | 5.94x | 1.75x |
| KellyCriterion | quantstats 0.0.81 | 55.60x | 8.39x | 4.10x |
| CommonSenseRatio | quantstats 0.0.81 | 73.89x | 10.61x | 4.56x |
| CompositeProfitabilityConsistencyIndex | quantstats 0.0.81 | 95.21x | 14.44x | 7.11x |
| ModifiedSharpeRatio | scipy 1.18.0 | 2.51x | 0.84x | 0.69x |
| ProbabilisticSharpeRatio | scipy 1.18.0 | 13.32x | 1.91x | 0.66x |
| DeflatedSharpeRatio | scipy 1.18.0 | 15.35x | 2.00x | 0.73x |
| ParametricValueAtRisk | scipy 1.18.0 | 13.83x | 1.93x | 0.39x |
| ParametricExpectedShortfall | scipy 1.18.0 | 22.12x | 2.83x | 0.52x |
| ConditionalDrawdownAtRisk | numpy 2.4.6 | 20.71x | 17.45x | 16.87x |
| EntropicValueAtRisk | scipy 1.18.0 | 6.47x | 1.36x | 1.04x |
| Exposure | quantstats 0.0.81 | 70.79x | 10.07x | 1.30x |
| EffectiveNumberOfBets | numpy 2.4.6 | 0.78x | 0.38x | 0.32x |
| Turnover | numpy 2.4.6 | 2.16x | 0.55x | 0.25x |

## Metric pipeline amortization

One Rust-owned P&L conversion and fan-out pass is compared with constructing the same eight TAFlow metric classes separately. This is an internal architecture comparison, not an external-oracle claim; results are gated by equality with the standalone public classes.

| **Metrics** | **Input** | **1k** | **10k** | **100k** |
|---|---|---:|---:|---:|
| 8 whole-history metrics | period P&L | 1.52x | 1.37x | 1.32x |

## Implementation interpretation

The public adapter performs one contiguous container conversion and releases the GIL for native bulk work. Rust bulk loops hoist semantic-domain validation and avoid per-observation result calculation. They deliberately retain chronological scalar accumulation for Welford moments, compensated sums, compounding, and drawdown state; these recurrences are not reassociated into SIMD reductions because scalar append, chunked extend, and batch extend must leave the same persistent state. Exact historical tails use cached linear-time selection rather than a full sort.

Consequently, array libraries can remain faster for simple one-shot reductions that use highly tuned vector kernels, while TAFlow's advantages are persistent O(1) continuation, cached O(1) reads, native streaming, and amortizing one semantic conversion across a metric pipeline.

## SharpeRatio execution profiles (100k observations)

These profiles separate native bulk processing from Python scalar/chunk boundary costs and cached reads.

| **Path** | **Median** |
|---|---:|
| native bulk | 0.496 ms |
| chunks 1024 | 0.614 ms |
| chunks 32 | 2.873 ms |
| scalar append | 15.154 ms |
| warmed continuation | 0.248 ms |
| cached compute | 73.8 ns |
