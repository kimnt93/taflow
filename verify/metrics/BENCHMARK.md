# Metrics benchmark

Generated: 2026-08-11

Public end-to-end semantic-factory `compute()` timings; every row passed the external correctness gate first.

| Metric | Observations | TAFlow median (ms) | Oracle median (ms) | Speedup |
|---|---:|---:|---:|---:|
| `TotalReturn` | 1,000 | 0.0122 | 0.0117 | 0.96x |
| `TotalReturn` | 10,000 | 0.1092 | 0.0264 | 0.24x |
| `TotalReturn` | 100,000 | 1.1136 | 0.1720 | 0.15x |
| `TotalReturn` | 1,000,000 | 11.3492 | 3.8786 | 0.34x |
| `AnnualizedReturn` | 1,000 | 0.0156 | 0.0133 | 0.85x |
| `AnnualizedReturn` | 10,000 | 0.1376 | 0.0282 | 0.21x |
| `AnnualizedReturn` | 100,000 | 1.3710 | 0.1701 | 0.12x |
| `AnnualizedReturn` | 1,000,000 | 13.9399 | 4.3468 | 0.31x |
| `AnnualizedVolatility` | 1,000 | 0.0073 | 0.0072 | 0.98x |
| `AnnualizedVolatility` | 10,000 | 0.0610 | 0.0233 | 0.38x |
| `AnnualizedVolatility` | 100,000 | 0.6031 | 0.1788 | 0.30x |
| `AnnualizedVolatility` | 1,000,000 | 6.0115 | 2.0044 | 0.33x |
| `MaximumDrawdown` | 1,000 | 0.0085 | 0.0236 | 2.76x |
| `MaximumDrawdown` | 10,000 | 0.0753 | 0.0834 | 1.11x |
| `MaximumDrawdown` | 100,000 | 0.7230 | 0.6667 | 0.92x |
| `MaximumDrawdown` | 1,000,000 | 7.2478 | 10.0608 | 1.39x |
| `DownsideDeviation` | 1,000 | 0.0054 | 0.0138 | 2.54x |
| `DownsideDeviation` | 10,000 | 0.0411 | 0.0295 | 0.72x |
| `DownsideDeviation` | 100,000 | 0.4008 | 0.1755 | 0.44x |
| `DownsideDeviation` | 1,000,000 | 3.9575 | 3.9963 | 1.01x |
| `SharpeRatio` | 1,000 | 0.0079 | 0.0200 | 2.52x |
| `SharpeRatio` | 10,000 | 0.0622 | 0.0525 | 0.84x |
| `SharpeRatio` | 100,000 | 0.6027 | 0.3461 | 0.57x |
| `SharpeRatio` | 1,000,000 | 5.8990 | 4.2678 | 0.72x |
| `SortinoRatio` | 1,000 | 0.0055 | 0.0198 | 3.59x |
| `SortinoRatio` | 10,000 | 0.0423 | 0.0437 | 1.03x |
| `SortinoRatio` | 100,000 | 0.4261 | 0.2668 | 0.63x |
| `SortinoRatio` | 1,000,000 | 4.3024 | 5.2652 | 1.22x |
| `CalmarRatio` | 1,000 | 0.0299 | 0.0405 | 1.35x |
| `CalmarRatio` | 10,000 | 0.2894 | 0.1141 | 0.39x |
| `CalmarRatio` | 100,000 | 2.8255 | 0.8417 | 0.30x |
| `CalmarRatio` | 1,000,000 | 28.9787 | 14.2280 | 0.49x |
| `OmegaRatio` | 1,000 | 0.0076 | 0.0681 | 8.93x |
| `OmegaRatio` | 10,000 | 0.0779 | 0.6066 | 7.78x |
| `OmegaRatio` | 100,000 | 0.8309 | 6.2929 | 7.57x |
| `OmegaRatio` | 1,000,000 | 8.0485 | 60.5044 | 7.52x |
| `HistoricalValueAtRisk` | 1,000 | 0.0168 | 0.0558 | 3.32x |
| `HistoricalValueAtRisk` | 10,000 | 0.2272 | 0.0781 | 0.34x |
| `HistoricalValueAtRisk` | 100,000 | 2.5576 | 0.8112 | 0.32x |
| `HistoricalValueAtRisk` | 1,000,000 | 37.5772 | 7.5935 | 0.20x |
| `HistoricalExpectedShortfall` | 1,000 | 0.0173 | 0.0120 | 0.69x |
| `HistoricalExpectedShortfall` | 10,000 | 0.2242 | 0.0265 | 0.12x |
| `HistoricalExpectedShortfall` | 100,000 | 2.5240 | 0.1702 | 0.07x |
| `HistoricalExpectedShortfall` | 1,000,000 | 37.2511 | 1.9911 | 0.05x |
| `TailRatio` | 1,000 | 0.0173 | 0.1159 | 6.69x |
| `TailRatio` | 10,000 | 0.2254 | 0.2143 | 0.95x |
| `TailRatio` | 100,000 | 2.7901 | 1.5854 | 0.57x |
| `TailRatio` | 1,000,000 | 36.9796 | 17.0599 | 0.46x |
| `TrackingError` | 1,000 | 0.0207 | 0.0200 | 0.97x |
| `TrackingError` | 10,000 | 0.1840 | 0.0342 | 0.19x |
| `TrackingError` | 100,000 | 1.7554 | 0.1973 | 0.11x |
| `TrackingError` | 1,000,000 | 18.1368 | 3.6057 | 0.20x |
| `InformationRatio` | 1,000 | 0.0234 | 0.0323 | 1.38x |
| `InformationRatio` | 10,000 | 0.2146 | 0.0692 | 0.32x |
| `InformationRatio` | 100,000 | 2.1875 | 0.4104 | 0.19x |
| `InformationRatio` | 1,000,000 | 21.0855 | 4.8732 | 0.23x |
| `Beta` | 1,000 | 0.0283 | 0.0182 | 0.64x |
| `Beta` | 10,000 | 0.2662 | 0.0612 | 0.23x |
| `Beta` | 100,000 | 2.6372 | 0.4944 | 0.19x |
| `Beta` | 1,000,000 | 26.6345 | 9.3382 | 0.35x |
| `Alpha` | 1,000 | 0.0296 | 0.0324 | 1.09x |
| `Alpha` | 10,000 | 0.2728 | 0.0992 | 0.36x |
| `Alpha` | 100,000 | 2.6716 | 0.7350 | 0.28x |
| `Alpha` | 1,000,000 | 27.2568 | 14.7984 | 0.54x |
| `CoefficientOfDetermination` | 1,000 | 0.0289 | 1.0989 | 38.01x |
| `CoefficientOfDetermination` | 10,000 | 0.2705 | 1.2310 | 4.55x |
| `CoefficientOfDetermination` | 100,000 | 3.7435 | 4.0920 | 1.09x |
| `CoefficientOfDetermination` | 1,000,000 | 26.8350 | 51.1568 | 1.91x |
| `CaptureRatio` | 1,000 | 0.0574 | 0.0263 | 0.46x |
| `CaptureRatio` | 10,000 | 0.3198 | 0.0633 | 0.20x |
| `CaptureRatio` | 100,000 | 3.1041 | 0.4033 | 0.13x |
| `CaptureRatio` | 1,000,000 | 31.8006 | 5.7733 | 0.18x |
| `UpMarketCaptureRatio` | 1,000 | 0.0285 | 0.0314 | 1.10x |
| `UpMarketCaptureRatio` | 10,000 | 0.2649 | 0.1266 | 0.48x |
| `UpMarketCaptureRatio` | 100,000 | 2.6298 | 1.0723 | 0.41x |
| `UpMarketCaptureRatio` | 1,000,000 | 26.7425 | 12.7850 | 0.48x |
| `DownMarketCaptureRatio` | 1,000 | 0.0281 | 0.0292 | 1.04x |
| `DownMarketCaptureRatio` | 10,000 | 0.2614 | 0.1285 | 0.49x |
| `DownMarketCaptureRatio` | 100,000 | 2.8498 | 1.0703 | 0.38x |
| `DownMarketCaptureRatio` | 1,000,000 | 26.7569 | 33.2858 | 1.24x |
| `UpDownCaptureRatio` | 1,000 | 0.0762 | 0.0591 | 0.77x |
| `UpDownCaptureRatio` | 10,000 | 0.7168 | 0.2585 | 0.36x |
| `UpDownCaptureRatio` | 100,000 | 7.3676 | 2.0646 | 0.28x |
| `UpDownCaptureRatio` | 1,000,000 | 71.4353 | 44.9777 | 0.63x |
| `UlcerIndex` | 1,000 | 0.0093 | 1.8103 | 194.04x |
| `UlcerIndex` | 10,000 | 0.0816 | 2.2798 | 27.93x |
| `UlcerIndex` | 100,000 | 0.7855 | 10.2727 | 13.08x |
| `UlcerIndex` | 1,000,000 | 8.0502 | 139.0847 | 17.28x |
| `UlcerPerformanceIndex` | 1,000 | 0.0261 | 1.8321 | 70.11x |
| `UlcerPerformanceIndex` | 10,000 | 0.2430 | 2.4439 | 10.06x |
| `UlcerPerformanceIndex` | 100,000 | 2.4256 | 10.2049 | 4.21x |
| `UlcerPerformanceIndex` | 1,000,000 | 24.3508 | 140.8350 | 5.78x |
| `RecoveryFactor` | 1,000 | 0.0156 | 1.3379 | 85.92x |
| `RecoveryFactor` | 10,000 | 0.1407 | 1.8336 | 13.03x |
| `RecoveryFactor` | 100,000 | 1.3498 | 7.4151 | 5.49x |
| `RecoveryFactor` | 1,000,000 | 13.8224 | 90.9625 | 6.58x |
| `GainToPainRatio` | 1,000 | 0.0070 | 0.9918 | 142.49x |
| `GainToPainRatio` | 10,000 | 0.0712 | 1.2071 | 16.95x |
| `GainToPainRatio` | 100,000 | 0.7326 | 4.0282 | 5.50x |
| `GainToPainRatio` | 1,000,000 | 7.4546 | 43.1995 | 5.80x |
| `PainIndex` | 1,000 | 0.0095 | 0.0276 | 2.92x |
| `PainIndex` | 10,000 | 0.0804 | 0.0859 | 1.07x |
| `PainIndex` | 100,000 | 0.8209 | 0.6451 | 0.79x |
| `PainIndex` | 1,000,000 | 7.9644 | 8.3391 | 1.05x |
| `StabilityOfTimeSeries` | 1,000 | 0.0224 | 0.4758 | 21.22x |
| `StabilityOfTimeSeries` | 10,000 | 0.2068 | 0.5350 | 2.59x |
| `StabilityOfTimeSeries` | 100,000 | 2.0118 | 2.3733 | 1.18x |
| `StabilityOfTimeSeries` | 1,000,000 | 20.4530 | 20.4495 | 1.00x |
| `BreakevenRate` | 1,000 | 0.0095 | 0.0089 | 0.94x |
| `BreakevenRate` | 10,000 | 0.0401 | 0.0155 | 0.39x |
| `BreakevenRate` | 100,000 | 0.3879 | 0.0717 | 0.18x |
| `BreakevenRate` | 1,000,000 | 4.1408 | 0.7645 | 0.18x |
| `WinRate` | 1,000 | 0.0074 | 0.4139 | 56.08x |
| `WinRate` | 10,000 | 0.0812 | 0.4489 | 5.52x |
| `WinRate` | 100,000 | 0.7320 | 1.2921 | 1.77x |
| `WinRate` | 1,000,000 | 7.2984 | 11.3183 | 1.55x |
| `AverageWin` | 1,000 | 0.0070 | 0.3611 | 51.61x |
| `AverageWin` | 10,000 | 0.0745 | 0.4439 | 5.96x |
| `AverageWin` | 100,000 | 0.7228 | 1.3053 | 1.81x |
| `AverageWin` | 1,000,000 | 7.4964 | 10.5717 | 1.41x |
| `AverageLoss` | 1,000 | 0.0075 | 0.3513 | 47.09x |
| `AverageLoss` | 10,000 | 0.0755 | 0.4407 | 5.84x |
| `AverageLoss` | 100,000 | 0.7785 | 1.2906 | 1.66x |
| `AverageLoss` | 1,000,000 | 7.2943 | 10.9310 | 1.50x |
| `PayoffRatio` | 1,000 | 0.0075 | 1.0141 | 135.45x |
| `PayoffRatio` | 10,000 | 0.0724 | 1.2744 | 17.61x |
| `PayoffRatio` | 100,000 | 0.7119 | 4.5106 | 6.34x |
| `PayoffRatio` | 1,000,000 | 7.2202 | 49.0539 | 6.79x |
| `ProfitFactor` | 1,000 | 0.0076 | 0.4429 | 57.98x |
| `ProfitFactor` | 10,000 | 0.0788 | 0.6211 | 7.88x |
| `ProfitFactor` | 100,000 | 0.8794 | 2.5647 | 2.92x |
| `ProfitFactor` | 1,000,000 | 8.2367 | 20.5260 | 2.49x |
| `LongestLosingStreak` | 1,000 | 0.0067 | 0.7708 | 114.77x |
| `LongestLosingStreak` | 10,000 | 0.0689 | 1.2988 | 18.85x |
| `LongestLosingStreak` | 100,000 | 0.7012 | 7.0811 | 10.10x |
| `LongestLosingStreak` | 1,000,000 | 7.0329 | 71.2746 | 10.13x |
| `LongestWinningStreak` | 1,000 | 0.0075 | 0.7583 | 100.54x |
| `LongestWinningStreak` | 10,000 | 0.0723 | 1.2984 | 17.97x |
| `LongestWinningStreak` | 100,000 | 0.7316 | 7.3790 | 10.09x |
| `LongestWinningStreak` | 1,000,000 | 7.1731 | 70.8981 | 9.88x |
| `NetProfit` | 1,000 | 0.0059 | 0.0046 | 0.77x |
| `NetProfit` | 10,000 | 0.0492 | 0.0067 | 0.14x |
| `NetProfit` | 100,000 | 0.4676 | 0.0260 | 0.06x |
| `NetProfit` | 1,000,000 | 4.7967 | 0.2535 | 0.05x |
| `GrossProfit` | 1,000 | 0.0066 | 0.0072 | 1.09x |
| `GrossProfit` | 10,000 | 0.0734 | 0.0470 | 0.64x |
| `GrossProfit` | 100,000 | 0.7296 | 0.4554 | 0.62x |
| `GrossProfit` | 1,000,000 | 7.1706 | 4.8317 | 0.67x |
| `GrossLoss` | 1,000 | 0.0069 | 0.0069 | 1.01x |
| `GrossLoss` | 10,000 | 0.0716 | 0.0463 | 0.65x |
| `GrossLoss` | 100,000 | 0.7139 | 0.4313 | 0.60x |
| `GrossLoss` | 1,000,000 | 7.1850 | 4.6270 | 0.64x |
| `Expectancy` | 1,000 | 0.0074 | 0.4239 | 57.15x |
| `Expectancy` | 10,000 | 0.0746 | 0.5625 | 7.54x |
| `Expectancy` | 100,000 | 0.7338 | 1.6426 | 2.24x |
| `Expectancy` | 1,000,000 | 7.2490 | 14.8209 | 2.04x |
| `KellyCriterion` | 1,000 | 0.0275 | 1.6832 | 61.31x |
| `KellyCriterion` | 10,000 | 0.2522 | 2.1736 | 8.62x |
| `KellyCriterion` | 100,000 | 2.4597 | 7.4837 | 3.04x |
| `KellyCriterion` | 1,000,000 | 25.1048 | 90.9571 | 3.62x |
| `CommonSenseRatio` | 1,000 | 0.0194 | 1.3441 | 69.31x |
| `CommonSenseRatio` | 10,000 | 0.2719 | 1.7607 | 6.48x |
| `CommonSenseRatio` | 100,000 | 2.9605 | 6.2590 | 2.11x |
| `CommonSenseRatio` | 1,000,000 | 35.3050 | 66.0026 | 1.87x |
| `CompositeProfitabilityConsistencyIndex` | 1,000 | 0.0203 | 2.1103 | 104.10x |
| `CompositeProfitabilityConsistencyIndex` | 10,000 | 0.1808 | 2.9070 | 16.08x |
| `CompositeProfitabilityConsistencyIndex` | 100,000 | 1.8027 | 10.2552 | 5.69x |
| `CompositeProfitabilityConsistencyIndex` | 1,000,000 | 18.7462 | 124.9381 | 6.66x |

## Representative execution profiles

Sharpe Ratio at 100,000 observations isolates the native bulk boundary, chunking, scalar append, warmed continuation, and cached compute paths.

| Path | Median (ms) | MAD (ms) |
|---|---:|---:|
| `native_bulk` | 0.5797 | 0.0025 |
| `chunks_32` | 2.6704 | 0.0332 |
| `chunks_1024` | 0.6736 | 0.0200 |
| `scalar_append` | 14.6076 | 0.1256 |
| `warmed_continuation` | 0.2909 | 0.0011 |
| `cached_compute` | 0.0001 | 0.0000 |

### Input-container conversion

Public end-to-end Sharpe Ratio construction and compute at 100,000 observations.

| Container | Median (ms) | MAD (ms) |
|---|---:|---:|
| `numpy` | 0.5955 | 0.0156 |
| `list` | 3.1871 | 0.0913 |
| `pandas` | 0.6151 | 0.0010 |
| `polars` | 0.5988 | 0.0093 |
| `arrow` | 0.6050 | 0.0147 |

## Exact-tail retained memory

Historical VaR, Historical Expected Shortfall, Tail Ratio, and Common Sense Ratio retain both chronological and sorted `f64` buffers after compute. The payload lower bound is 16 bytes per observation, excluding vector capacity and allocator overhead.

| Observations | Retained `f64` payload lower bound |
|---:|---:|
| 1,000 | 0.015 MiB |
| 10,000 | 0.153 MiB |
| 100,000 | 1.526 MiB |
| 1,000,000 | 15.259 MiB |

Environment: Python 3.12.3, NumPy 2.4.6, Empyrical Reloaded 0.5.12, QuantStats 0.0.81; OS Linux-6.18.7-76061807-generic-x86_64-with-glibc2.39; machine x86_64; rustc 1.97.1 (8bab26f4f 2026-07-14); release extension `/home/kim/Documents/me/taflow/python/taflow/_native.cpython-312-x86_64-linux-gnu.so`.
