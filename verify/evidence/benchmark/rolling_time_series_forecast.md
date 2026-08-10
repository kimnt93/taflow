# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.87M | 0.016 | 61.15M | 0.043 | 2.47× | 2.66× |
| 10,000 | 0.138 | 72.46M | 0.135 | 73.81M | 0.150 | 1.09× | 1.11× |
| 100,000 | 1.332 | 75.09M | 1.278 | 78.25M | 1.249 | 0.94× | 0.98× |
| 1,000,000 | 13.439 | 74.41M | 13.292 | 75.23M | 12.400 | 0.92× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.110 | 0.61× |
| 1 | 5 | 0.320 | 0.464 | 1.45× |
| 1 | 10 | 0.469 | 0.948 | 2.02× |
| 10 | 1 | 0.049 | 0.093 | 1.90× |
| 10 | 5 | 0.230 | 0.436 | 1.89× |
| 10 | 10 | 0.458 | 0.926 | 2.02× |
| 100 | 1 | 0.055 | 0.092 | 1.68× |
| 100 | 5 | 0.233 | 0.449 | 1.93× |
| 100 | 10 | 0.496 | 0.991 | 2.00× |
| 1,000 | 1 | 0.083 | 0.121 | 1.46× |
| 1,000 | 5 | 0.269 | 0.549 | 2.04× |
| 1,000 | 10 | 0.528 | 1.102 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
