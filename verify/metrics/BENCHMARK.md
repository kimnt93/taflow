# Metrics benchmark

Generated: 2026-08-12

Public end-to-end semantic-factory `compute()` timings for 52 benchmark-eligible metrics; every row passed the external correctness gate first.

| Metric | Oracle source function | Correctness | Observations | TAFlow median (ms) | Oracle median (ms) | Speedup |
|---|---|---:|---:|---:|---:|---:|
| `TotalReturn` | [`empyrical.stats.cum_returns_final`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0135 | 0.0129 | 0.96x |
| `TotalReturn` | [`empyrical.stats.cum_returns_final`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.1288 | 0.0279 | 0.22x |
| `TotalReturn` | [`empyrical.stats.cum_returns_final`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 1.2067 | 0.1756 | 0.15x |
| `TotalReturn` | [`empyrical.stats.cum_returns_final`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 12.3636 | 4.9589 | 0.40x |
| `AnnualizedReturn` | [`empyrical.stats.annual_return`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0164 | 0.0133 | 0.81x |
| `AnnualizedReturn` | [`empyrical.stats.annual_return`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.1475 | 0.0365 | 0.25x |
| `AnnualizedReturn` | [`empyrical.stats.annual_return`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 1.5000 | 0.1835 | 0.12x |
| `AnnualizedReturn` | [`empyrical.stats.annual_return`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 14.6107 | 4.7428 | 0.32x |
| `AnnualizedVolatility` | [`empyrical.stats.annual_volatility`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0082 | 0.0077 | 0.94x |
| `AnnualizedVolatility` | [`empyrical.stats.annual_volatility`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.0689 | 0.0248 | 0.36x |
| `AnnualizedVolatility` | [`empyrical.stats.annual_volatility`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 0.6364 | 0.1871 | 0.29x |
| `AnnualizedVolatility` | [`empyrical.stats.annual_volatility`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 6.6115 | 2.1754 | 0.33x |
| `MaximumDrawdown` | [`empyrical.stats.max_drawdown`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0094 | 0.0249 | 2.64x |
| `MaximumDrawdown` | [`empyrical.stats.max_drawdown`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.0794 | 0.0864 | 1.09x |
| `MaximumDrawdown` | [`empyrical.stats.max_drawdown`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 0.8142 | 0.7150 | 0.88x |
| `MaximumDrawdown` | [`empyrical.stats.max_drawdown`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 8.3377 | 11.3126 | 1.36x |
| `DownsideDeviation` | [`empyrical.stats.downside_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0063 | 0.0141 | 2.23x |
| `DownsideDeviation` | [`empyrical.stats.downside_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.0464 | 0.0321 | 0.69x |
| `DownsideDeviation` | [`empyrical.stats.downside_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 0.4310 | 0.1792 | 0.42x |
| `DownsideDeviation` | [`empyrical.stats.downside_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 4.5420 | 4.5858 | 1.01x |
| `SharpeRatio` | [`empyrical.stats.sharpe_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0081 | 0.0229 | 2.83x |
| `SharpeRatio` | [`empyrical.stats.sharpe_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.0661 | 0.0530 | 0.80x |
| `SharpeRatio` | [`empyrical.stats.sharpe_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 0.6782 | 0.3949 | 0.58x |
| `SharpeRatio` | [`empyrical.stats.sharpe_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 6.5594 | 4.6748 | 0.71x |
| `SortinoRatio` | [`empyrical.stats.sortino_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0061 | 0.0218 | 3.60x |
| `SortinoRatio` | [`empyrical.stats.sortino_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.0462 | 0.0466 | 1.01x |
| `SortinoRatio` | [`empyrical.stats.sortino_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 0.4555 | 0.2797 | 0.61x |
| `SortinoRatio` | [`empyrical.stats.sortino_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 4.5707 | 5.0883 | 1.11x |
| `CalmarRatio` | [`empyrical.stats.calmar_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0323 | 0.0413 | 1.28x |
| `CalmarRatio` | [`empyrical.stats.calmar_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2967 | 0.1186 | 0.40x |
| `CalmarRatio` | [`empyrical.stats.calmar_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 3.0900 | 0.8725 | 0.28x |
| `CalmarRatio` | [`empyrical.stats.calmar_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 30.0885 | 15.0701 | 0.50x |
| `OmegaRatio` | [`empyrical.stats.omega_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0105 | 0.0754 | 7.19x |
| `OmegaRatio` | [`empyrical.stats.omega_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.0884 | 0.6819 | 7.71x |
| `OmegaRatio` | [`empyrical.stats.omega_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 0.8562 | 6.7778 | 7.92x |
| `OmegaRatio` | [`empyrical.stats.omega_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 8.9071 | 66.3438 | 7.45x |
| `HistoricalValueAtRisk` | [`empyrical.stats.value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0182 | 0.0564 | 3.11x |
| `HistoricalValueAtRisk` | [`empyrical.stats.value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2544 | 0.0814 | 0.32x |
| `HistoricalValueAtRisk` | [`empyrical.stats.value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.8423 | 0.8753 | 0.31x |
| `HistoricalValueAtRisk` | [`empyrical.stats.value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 39.9431 | 8.4578 | 0.21x |
| `HistoricalExpectedShortfall` | [`empyrical.stats.conditional_value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0186 | 0.0118 | 0.63x |
| `HistoricalExpectedShortfall` | [`empyrical.stats.conditional_value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2519 | 0.0281 | 0.11x |
| `HistoricalExpectedShortfall` | [`empyrical.stats.conditional_value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.8383 | 0.1864 | 0.07x |
| `HistoricalExpectedShortfall` | [`empyrical.stats.conditional_value_at_risk`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 39.8796 | 2.2333 | 0.06x |
| `TailRatio` | [`empyrical.stats.tail_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0191 | 0.1188 | 6.21x |
| `TailRatio` | [`empyrical.stats.tail_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2488 | 0.2206 | 0.89x |
| `TailRatio` | [`empyrical.stats.tail_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.8030 | 1.6439 | 0.59x |
| `TailRatio` | [`empyrical.stats.tail_ratio`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 39.8982 | 18.3997 | 0.46x |
| `TrackingError` | [`numpy.std`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000 | 0.0212 | 0.0181 | 0.85x |
| `TrackingError` | [`numpy.std`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 10,000 | 0.1888 | 0.0391 | 0.21x |
| `TrackingError` | [`numpy.std`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 100,000 | 1.8449 | 0.2065 | 0.11x |
| `TrackingError` | [`numpy.std`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000,000 | 19.6579 | 4.7317 | 0.24x |
| `InformationRatio` | [`empyrical.stats.excess_sharpe`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0213 | 0.0308 | 1.45x |
| `InformationRatio` | [`empyrical.stats.excess_sharpe`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.1916 | 0.0704 | 0.37x |
| `InformationRatio` | [`empyrical.stats.excess_sharpe`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.0038 | 0.5094 | 0.25x |
| `InformationRatio` | [`empyrical.stats.excess_sharpe`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 19.2398 | 4.9610 | 0.26x |
| `Beta` | [`empyrical.stats.beta_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0298 | 0.0193 | 0.65x |
| `Beta` | [`empyrical.stats.beta_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2865 | 0.0698 | 0.24x |
| `Beta` | [`empyrical.stats.beta_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.7560 | 0.5166 | 0.19x |
| `Beta` | [`empyrical.stats.beta_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 28.1202 | 9.9595 | 0.35x |
| `Alpha` | [`empyrical.stats.alpha_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0313 | 0.0328 | 1.05x |
| `Alpha` | [`empyrical.stats.alpha_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2875 | 0.1014 | 0.35x |
| `Alpha` | [`empyrical.stats.alpha_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.7864 | 0.7608 | 0.27x |
| `Alpha` | [`empyrical.stats.alpha_aligned`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 29.4831 | 16.3369 | 0.55x |
| `CoefficientOfDetermination` | [`quantstats.stats.r_squared`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0310 | 1.0653 | 34.33x |
| `CoefficientOfDetermination` | [`quantstats.stats.r_squared`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.2970 | 1.2668 | 4.27x |
| `CoefficientOfDetermination` | [`quantstats.stats.r_squared`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 2.8088 | 4.4516 | 1.58x |
| `CoefficientOfDetermination` | [`quantstats.stats.r_squared`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 28.5404 | 60.2507 | 2.11x |
| `CaptureRatio` | [`empyrical.stats.capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0623 | 0.0278 | 0.45x |
| `CaptureRatio` | [`empyrical.stats.capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.3620 | 0.0648 | 0.18x |
| `CaptureRatio` | [`empyrical.stats.capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 3.3265 | 0.4224 | 0.13x |
| `CaptureRatio` | [`empyrical.stats.capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 33.9019 | 7.0766 | 0.21x |
| `UpMarketCaptureRatio` | [`empyrical.stats.up_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0316 | 0.0320 | 1.01x |
| `UpMarketCaptureRatio` | [`empyrical.stats.up_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2748 | 0.1365 | 0.50x |
| `UpMarketCaptureRatio` | [`empyrical.stats.up_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.7300 | 1.2141 | 0.44x |
| `UpMarketCaptureRatio` | [`empyrical.stats.up_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 29.5686 | 13.2945 | 0.45x |
| `DownMarketCaptureRatio` | [`empyrical.stats.down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0322 | 0.0319 | 0.99x |
| `DownMarketCaptureRatio` | [`empyrical.stats.down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2984 | 0.1312 | 0.44x |
| `DownMarketCaptureRatio` | [`empyrical.stats.down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.8360 | 1.1724 | 0.41x |
| `DownMarketCaptureRatio` | [`empyrical.stats.down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 28.0979 | 35.5710 | 1.27x |
| `UpDownCaptureRatio` | [`empyrical.stats.up_down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0825 | 0.0656 | 0.80x |
| `UpDownCaptureRatio` | [`empyrical.stats.up_down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.7463 | 0.2585 | 0.35x |
| `UpDownCaptureRatio` | [`empyrical.stats.up_down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 7.5651 | 2.2991 | 0.30x |
| `UpDownCaptureRatio` | [`empyrical.stats.up_down_capture`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 79.7910 | 48.5344 | 0.61x |
| `UlcerIndex` | [`quantstats.stats.ulcer_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0105 | 1.8315 | 174.24x |
| `UlcerIndex` | [`quantstats.stats.ulcer_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0862 | 2.4098 | 27.97x |
| `UlcerIndex` | [`quantstats.stats.ulcer_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.9209 | 11.8648 | 12.88x |
| `UlcerIndex` | [`quantstats.stats.ulcer_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 8.4668 | 156.1982 | 18.45x |
| `UlcerPerformanceIndex` | [`quantstats.stats.ulcer_performance_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0293 | 1.9242 | 65.58x |
| `UlcerPerformanceIndex` | [`quantstats.stats.ulcer_performance_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.2736 | 2.7055 | 9.89x |
| `UlcerPerformanceIndex` | [`quantstats.stats.ulcer_performance_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 2.6180 | 11.3277 | 4.33x |
| `UlcerPerformanceIndex` | [`quantstats.stats.ulcer_performance_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 28.4230 | 165.9060 | 5.84x |
| `RecoveryFactor` | [`quantstats.stats.recovery_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0150 | 1.5119 | 100.69x |
| `RecoveryFactor` | [`quantstats.stats.recovery_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.1399 | 1.9616 | 14.02x |
| `RecoveryFactor` | [`quantstats.stats.recovery_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 1.3680 | 8.5588 | 6.26x |
| `RecoveryFactor` | [`quantstats.stats.recovery_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 13.6193 | 101.5063 | 7.45x |
| `GainToPainRatio` | [`quantstats.stats.gain_to_pain_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0074 | 1.0284 | 138.52x |
| `GainToPainRatio` | [`quantstats.stats.gain_to_pain_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0751 | 1.3373 | 17.81x |
| `GainToPainRatio` | [`quantstats.stats.gain_to_pain_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.7569 | 4.4259 | 5.85x |
| `GainToPainRatio` | [`quantstats.stats.gain_to_pain_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 7.7256 | 48.3482 | 6.26x |
| `PainIndex` | [`PerformanceAnalytics::PainIndex`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 1,000 | 0.0096 | 0.0294 | 3.06x |
| `PainIndex` | [`PerformanceAnalytics::PainIndex`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 10,000 | 0.0835 | 0.0854 | 1.02x |
| `PainIndex` | [`PerformanceAnalytics::PainIndex`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 100,000 | 0.8420 | 0.7077 | 0.84x |
| `PainIndex` | [`PerformanceAnalytics::PainIndex`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 1,000,000 | 8.6395 | 9.0731 | 1.05x |
| `StabilityOfTimeSeries` | [`empyrical.stats.stability_of_timeseries`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000 | 0.0236 | 0.4477 | 18.95x |
| `StabilityOfTimeSeries` | [`empyrical.stats.stability_of_timeseries`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 10,000 | 0.2141 | 0.5976 | 2.79x |
| `StabilityOfTimeSeries` | [`empyrical.stats.stability_of_timeseries`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 100,000 | 2.0913 | 2.5290 | 1.21x |
| `StabilityOfTimeSeries` | [`empyrical.stats.stability_of_timeseries`](https://github.com/stefan-jansen/empyrical-reloaded/blob/0.5.12/src/empyrical/stats.py) | **MATCH** | 1,000,000 | 22.3922 | 22.6705 | 1.01x |
| `BreakevenRate` | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000 | 0.0102 | 0.0099 | 0.97x |
| `BreakevenRate` | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 10,000 | 0.0434 | 0.0168 | 0.39x |
| `BreakevenRate` | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 100,000 | 0.4114 | 0.0795 | 0.19x |
| `BreakevenRate` | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000,000 | 4.2163 | 0.7494 | 0.18x |
| `WinRate` | [`quantstats.stats.win_rate`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0079 | 0.3618 | 46.04x |
| `WinRate` | [`quantstats.stats.win_rate`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0781 | 0.4783 | 6.12x |
| `WinRate` | [`quantstats.stats.win_rate`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.7537 | 1.5482 | 2.05x |
| `WinRate` | [`quantstats.stats.win_rate`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 7.8967 | 12.3203 | 1.56x |
| `AverageWin` | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0073 | 0.3573 | 48.94x |
| `AverageWin` | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0775 | 0.4563 | 5.89x |
| `AverageWin` | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.7881 | 1.4151 | 1.80x |
| `AverageWin` | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 7.9796 | 13.0498 | 1.64x |
| `AverageLoss` | [`quantstats.stats.avg_loss`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0080 | 0.3829 | 47.79x |
| `AverageLoss` | [`quantstats.stats.avg_loss`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0782 | 0.4684 | 5.99x |
| `AverageLoss` | [`quantstats.stats.avg_loss`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.8157 | 1.5887 | 1.95x |
| `AverageLoss` | [`quantstats.stats.avg_loss`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 8.4819 | 12.7790 | 1.51x |
| `PayoffRatio` | [`quantstats.stats.payoff_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0096 | 1.0575 | 110.01x |
| `PayoffRatio` | [`quantstats.stats.payoff_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0789 | 1.4305 | 18.12x |
| `PayoffRatio` | [`quantstats.stats.payoff_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.7622 | 4.9090 | 6.44x |
| `PayoffRatio` | [`quantstats.stats.payoff_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 7.9325 | 55.1638 | 6.95x |
| `ProfitFactor` | [`quantstats.stats.profit_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0076 | 0.4817 | 63.67x |
| `ProfitFactor` | [`quantstats.stats.profit_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0848 | 0.6709 | 7.91x |
| `ProfitFactor` | [`quantstats.stats.profit_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.8471 | 2.5832 | 3.05x |
| `ProfitFactor` | [`quantstats.stats.profit_factor`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 8.7226 | 23.1216 | 2.65x |
| `LongestLosingStreak` | [`quantstats.stats.consecutive_losses`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0083 | 0.8543 | 103.11x |
| `LongestLosingStreak` | [`quantstats.stats.consecutive_losses`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0781 | 1.5271 | 19.55x |
| `LongestLosingStreak` | [`quantstats.stats.consecutive_losses`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.7427 | 8.1638 | 10.99x |
| `LongestLosingStreak` | [`quantstats.stats.consecutive_losses`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 8.0277 | 80.0284 | 9.97x |
| `LongestWinningStreak` | [`quantstats.stats.consecutive_wins`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0072 | 0.8264 | 114.54x |
| `LongestWinningStreak` | [`quantstats.stats.consecutive_wins`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0790 | 1.4169 | 17.94x |
| `LongestWinningStreak` | [`quantstats.stats.consecutive_wins`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.7593 | 8.3582 | 11.01x |
| `LongestWinningStreak` | [`quantstats.stats.consecutive_wins`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 7.9084 | 80.2342 | 10.15x |
| `NetProfit` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000 | 0.0064 | 0.0048 | 0.75x |
| `NetProfit` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 10,000 | 0.0516 | 0.0070 | 0.13x |
| `NetProfit` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 100,000 | 0.5019 | 0.0277 | 0.06x |
| `NetProfit` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000,000 | 5.0255 | 0.2338 | 0.05x |
| `GrossProfit` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000 | 0.0075 | 0.0074 | 0.99x |
| `GrossProfit` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 10,000 | 0.0754 | 0.0517 | 0.69x |
| `GrossProfit` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 100,000 | 0.7680 | 0.4649 | 0.61x |
| `GrossProfit` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000,000 | 7.9572 | 4.8660 | 0.61x |
| `GrossLoss` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000 | 0.0073 | 0.0077 | 1.05x |
| `GrossLoss` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 10,000 | 0.0758 | 0.0511 | 0.67x |
| `GrossLoss` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 100,000 | 0.7879 | 0.4750 | 0.60x |
| `GrossLoss` | [`numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000,000 | 7.9108 | 5.7718 | 0.73x |
| `Expectancy` | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0080 | 0.4636 | 57.94x |
| `Expectancy` | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0769 | 0.5961 | 7.75x |
| `Expectancy` | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.7802 | 1.8141 | 2.33x |
| `Expectancy` | [`quantstats.stats.avg_win`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 8.0336 | 16.6569 | 2.07x |
| `KellyCriterion` | [`quantstats.stats.kelly_criterion`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0274 | 1.6980 | 62.06x |
| `KellyCriterion` | [`quantstats.stats.kelly_criterion`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.2688 | 2.2469 | 8.36x |
| `KellyCriterion` | [`quantstats.stats.kelly_criterion`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 2.6931 | 8.1135 | 3.01x |
| `KellyCriterion` | [`quantstats.stats.kelly_criterion`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 26.6414 | 102.9288 | 3.86x |
| `CommonSenseRatio` | [`quantstats.stats.common_sense_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0213 | 1.4188 | 66.66x |
| `CommonSenseRatio` | [`quantstats.stats.common_sense_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.2925 | 1.8542 | 6.34x |
| `CommonSenseRatio` | [`quantstats.stats.common_sense_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 3.4884 | 7.1368 | 2.05x |
| `CommonSenseRatio` | [`quantstats.stats.common_sense_ratio`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 39.2261 | 71.8451 | 1.83x |
| `CompositeProfitabilityConsistencyIndex` | [`quantstats.stats.cpc_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0211 | 2.3108 | 109.50x |
| `CompositeProfitabilityConsistencyIndex` | [`quantstats.stats.cpc_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.1907 | 3.0881 | 16.20x |
| `CompositeProfitabilityConsistencyIndex` | [`quantstats.stats.cpc_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 1.9252 | 12.6955 | 6.59x |
| `CompositeProfitabilityConsistencyIndex` | [`quantstats.stats.cpc_index`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 19.6339 | 138.2205 | 7.04x |
| `ModifiedSharpeRatio` | [`PerformanceAnalytics::SharpeRatio.modified(FUN='VaR')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 1,000 | 0.0535 | 0.1433 | 2.68x |
| `ModifiedSharpeRatio` | [`PerformanceAnalytics::SharpeRatio.modified(FUN='VaR')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 10,000 | 0.5250 | 0.4793 | 0.91x |
| `ModifiedSharpeRatio` | [`PerformanceAnalytics::SharpeRatio.modified(FUN='VaR')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 100,000 | 4.9662 | 3.7830 | 0.76x |
| `ModifiedSharpeRatio` | [`PerformanceAnalytics::SharpeRatio.modified(FUN='VaR')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 1,000,000 | 50.9835 | 40.0185 | 0.78x |
| `ProbabilisticSharpeRatio` | [`vectorbt probabilistic Sharpe kernel + scipy.stats.norm.cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 1,000 | 0.0726 | 1.0119 | 13.94x |
| `ProbabilisticSharpeRatio` | [`vectorbt probabilistic Sharpe kernel + scipy.stats.norm.cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 10,000 | 0.7020 | 1.4297 | 2.04x |
| `ProbabilisticSharpeRatio` | [`vectorbt probabilistic Sharpe kernel + scipy.stats.norm.cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 100,000 | 7.6131 | 5.2459 | 0.69x |
| `ProbabilisticSharpeRatio` | [`vectorbt probabilistic Sharpe kernel + scipy.stats.norm.cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 1,000,000 | 75.8125 | 54.0475 | 0.71x |
| `DeflatedSharpeRatio` | [`vectorbt deflated Sharpe kernel + scipy.stats.norm.ppf/cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 1,000 | 0.0818 | 1.2193 | 14.91x |
| `DeflatedSharpeRatio` | [`vectorbt deflated Sharpe kernel + scipy.stats.norm.ppf/cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 10,000 | 0.7832 | 1.5785 | 2.02x |
| `DeflatedSharpeRatio` | [`vectorbt deflated Sharpe kernel + scipy.stats.norm.ppf/cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 100,000 | 8.2084 | 5.4348 | 0.66x |
| `DeflatedSharpeRatio` | [`vectorbt deflated Sharpe kernel + scipy.stats.norm.ppf/cdf`](https://github.com/polakowo/vectorbt/blob/993ceca7116fc8e55f4cd3a36fe43d83dab62b27/vectorbt/returns/metrics.py) | **MATCH** | 1,000,000 | 82.8205 | 52.3081 | 0.63x |
| `ParametricValueAtRisk` | [`scipy.stats.norm.ppf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 1,000 | 0.0079 | 0.0940 | 11.87x |
| `ParametricValueAtRisk` | [`scipy.stats.norm.ppf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 10,000 | 0.0666 | 0.1041 | 1.56x |
| `ParametricValueAtRisk` | [`scipy.stats.norm.ppf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 100,000 | 0.6454 | 0.2260 | 0.35x |
| `ParametricValueAtRisk` | [`scipy.stats.norm.ppf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 1,000,000 | 6.5268 | 2.2006 | 0.34x |
| `ParametricExpectedShortfall` | [`scipy.stats.norm.ppf/pdf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 1,000 | 0.0081 | 0.1558 | 19.34x |
| `ParametricExpectedShortfall` | [`scipy.stats.norm.ppf/pdf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 10,000 | 0.0661 | 0.1664 | 2.52x |
| `ParametricExpectedShortfall` | [`scipy.stats.norm.ppf/pdf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 100,000 | 0.6393 | 0.2789 | 0.44x |
| `ParametricExpectedShortfall` | [`scipy.stats.norm.ppf/pdf + numpy.std`](https://github.com/scipy/scipy/blob/v1.18.0/scipy/stats/_continuous_distns.py) | **MATCH** | 1,000,000 | 6.3512 | 2.5830 | 0.41x |
| `ConditionalDrawdownAtRisk` | [`PerformanceAnalytics::CDD(method='discrete')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 1,000 | 0.0138 | 0.3162 | 22.93x |
| `ConditionalDrawdownAtRisk` | [`PerformanceAnalytics::CDD(method='discrete')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 10,000 | 0.1188 | 2.3415 | 19.71x |
| `ConditionalDrawdownAtRisk` | [`PerformanceAnalytics::CDD(method='discrete')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 100,000 | 1.1836 | 22.9513 | 19.39x |
| `ConditionalDrawdownAtRisk` | [`PerformanceAnalytics::CDD(method='discrete')`](https://cran.r-project.org/src/contrib/Archive/PerformanceAnalytics/PerformanceAnalytics_2.1.0.tar.gz) | **MATCH** | 1,000,000 | 12.0063 | 226.8374 | 18.89x |
| `EntropicValueAtRisk` | [`riskfolio.RiskFunctions.EVaR_Hist`](https://github.com/dcajasn/Riskfolio-Lib/blob/632a9e48fbaf2b9f8e83864a492332364b6ed32c/riskfolio/src/RiskFunctions.py) | **MATCH** | 1,000 | 0.3149 | 2.2664 | 7.20x |
| `EntropicValueAtRisk` | [`riskfolio.RiskFunctions.EVaR_Hist`](https://github.com/dcajasn/Riskfolio-Lib/blob/632a9e48fbaf2b9f8e83864a492332364b6ed32c/riskfolio/src/RiskFunctions.py) | **MATCH** | 10,000 | 3.1419 | 4.9333 | 1.57x |
| `EntropicValueAtRisk` | [`riskfolio.RiskFunctions.EVaR_Hist`](https://github.com/dcajasn/Riskfolio-Lib/blob/632a9e48fbaf2b9f8e83864a492332364b6ed32c/riskfolio/src/RiskFunctions.py) | **MATCH** | 100,000 | 31.5673 | 25.7998 | 0.82x |
| `EntropicValueAtRisk` | [`riskfolio.RiskFunctions.EVaR_Hist`](https://github.com/dcajasn/Riskfolio-Lib/blob/632a9e48fbaf2b9f8e83864a492332364b6ed32c/riskfolio/src/RiskFunctions.py) | **MATCH** | 1,000,000 | 322.9790 | 363.0038 | 1.12x |
| `Exposure` | [`quantstats.stats.exposure`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000 | 0.0050 | 0.3573 | 72.02x |
| `Exposure` | [`quantstats.stats.exposure`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 10,000 | 0.0365 | 0.3741 | 10.25x |
| `Exposure` | [`quantstats.stats.exposure`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 100,000 | 0.3464 | 0.4838 | 1.40x |
| `Exposure` | [`quantstats.stats.exposure`](https://github.com/ranaroussi/quantstats/blob/v0.0.81/quantstats/stats.py) | **MATCH** | 1,000,000 | 3.5028 | 2.7497 | 0.79x |
| `EffectiveNumberOfBets` | [`numpy.linalg.eigh + numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/linalg/_linalg.py) | **MATCH** | 1,000 | 0.0227 | 0.0180 | 0.79x |
| `EffectiveNumberOfBets` | [`numpy.linalg.eigh + numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/linalg/_linalg.py) | **MATCH** | 10,000 | 0.2123 | 0.0843 | 0.40x |
| `EffectiveNumberOfBets` | [`numpy.linalg.eigh + numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/linalg/_linalg.py) | **MATCH** | 100,000 | 2.1825 | 0.6891 | 0.32x |
| `EffectiveNumberOfBets` | [`numpy.linalg.eigh + numpy.sum`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/linalg/_linalg.py) | **MATCH** | 1,000,000 | 21.9826 | 9.2692 | 0.42x |
| `Turnover` | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000 | 0.0047 | 0.0109 | 2.34x |
| `Turnover` | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 10,000 | 0.0367 | 0.0185 | 0.50x |
| `Turnover` | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 100,000 | 0.3399 | 0.1067 | 0.31x |
| `Turnover` | [`numpy.mean`](https://github.com/numpy/numpy/blob/v2.4.6/numpy/_core/fromnumeric.py) | **MATCH** | 1,000,000 | 3.5461 | 1.6648 | 0.47x |

## Representative execution profiles

Sharpe Ratio at 100,000 observations isolates the native bulk boundary, chunking, scalar append, warmed continuation, and cached compute paths.

| Path | Median (ms) | MAD (ms) |
|---|---:|---:|
| `native_bulk` | 0.6720 | 0.0401 |
| `chunks_32` | 3.0901 | 0.1890 |
| `chunks_1024` | 0.8002 | 0.0394 |
| `scalar_append` | 15.6471 | 0.1048 |
| `warmed_continuation` | 0.3355 | 0.0011 |
| `cached_compute` | 0.0001 | 0.0000 |

### Input-container conversion

Public end-to-end Sharpe Ratio construction and compute at 100,000 observations.

| Container | Median (ms) | MAD (ms) |
|---|---:|---:|
| `numpy` | 0.6475 | 0.0181 |
| `list` | 3.2938 | 0.2785 |
| `pandas` | 0.6517 | 0.0069 |
| `polars` | 0.6336 | 0.0186 |
| `arrow` | 0.6482 | 0.0168 |

## Exact-tail retained memory

Historical VaR, Historical Expected Shortfall, Tail Ratio, and Common Sense Ratio retain both chronological and sorted `f64` buffers after compute. Conditional Drawdown at Risk retains two buffers per drawdown episode. Entropic Value at Risk retains one `f64` per usable return. The payload estimates exclude vector capacity and allocator overhead.

| Observations/episodes | Exact order-statistics and CDaR lower bound | Entropic VaR lower bound |
|---:|---:|---:|
| 1,000 | 0.015 MiB | 0.008 MiB |
| 10,000 | 0.153 MiB | 0.076 MiB |
| 100,000 | 1.526 MiB | 0.763 MiB |
| 1,000,000 | 15.259 MiB | 7.629 MiB |

Environment: Python 3.12.3, NumPy 2.4.6, Empyrical Reloaded 0.5.12, QuantStats 0.0.81; SciPy 1.18.0; OS Linux-6.18.7-76061807-generic-x86_64-with-glibc2.39; machine x86_64; rustc 1.97.1 (8bab26f4f 2026-07-14); release extension `/home/kim/Documents/me/taflow/python/taflow/_native.cpython-312-x86_64-linux-gnu.so`.
