# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 172.89M | 0.005 | 207.35M | 0.034 | 5.95× | 7.14× |
| 10,000 | 0.025 | 397.57M | 0.022 | 444.48M | 0.050 | 2.00× | 2.24× |
| 100,000 | 0.228 | 439.45M | 0.187 | 534.79M | 0.151 | 0.66× | 0.80× |
| 1,000,000 | 2.778 | 359.93M | 2.159 | 463.28M | 1.875 | 0.67× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.162 | 1.66× |
| 1 | 5 | 0.368 | 1.784 | 4.85× |
| 1 | 10 | 0.650 | 1.888 | 2.91× |
| 10 | 1 | 0.055 | 0.106 | 1.92× |
| 10 | 5 | 0.333 | 0.489 | 1.47× |
| 10 | 10 | 0.539 | 1.274 | 2.36× |
| 100 | 1 | 0.055 | 0.098 | 1.77× |
| 100 | 5 | 0.254 | 0.452 | 1.78× |
| 100 | 10 | 0.496 | 1.104 | 2.23× |
| 1,000 | 1 | 0.059 | 0.105 | 1.80× |
| 1,000 | 5 | 0.242 | 0.463 | 1.91× |
| 1,000 | 10 | 0.491 | 1.088 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
