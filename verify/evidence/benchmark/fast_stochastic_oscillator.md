# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.10M | 0.023 | 43.87M | 0.049 | 2.51× | 2.16× |
| 10,000 | 0.174 | 57.32M | 0.162 | 61.89M | 0.138 | 0.79× | 0.85× |
| 100,000 | 1.638 | 61.03M | 1.624 | 61.58M | 1.084 | 0.66× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.120 | 1.95× |
| 1 | 5 | 0.306 | 0.524 | 1.71× |
| 1 | 10 | 0.505 | 1.069 | 2.12× |
| 10 | 1 | 0.055 | 0.099 | 1.82× |
| 10 | 5 | 0.312 | 0.535 | 1.71× |
| 10 | 10 | 0.524 | 1.105 | 2.11× |
| 100 | 1 | 0.062 | 0.104 | 1.67× |
| 100 | 5 | 0.259 | 0.477 | 1.84× |
| 100 | 10 | 0.530 | 1.042 | 1.96× |
| 1,000 | 1 | 0.074 | 0.133 | 1.81× |
| 1,000 | 5 | 0.273 | 0.568 | 2.08× |
| 1,000 | 10 | 0.537 | 1.200 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
