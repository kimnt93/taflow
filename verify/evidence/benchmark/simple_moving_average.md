# SimpleMovingAverage benchmark (`SMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.01M | 0.005 | 182.09M | 0.043 | 6.96× | 7.82× |
| 10,000 | 0.030 | 337.58M | 0.026 | 387.93M | 0.059 | 2.00× | 2.30× |
| 100,000 | 0.253 | 394.65M | 0.213 | 470.45M | 0.235 | 0.93× | 1.11× |
| 1,000,000 | 2.733 | 365.84M | 2.397 | 417.20M | 2.221 | 0.81× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.137 | 2.22× |
| 1 | 5 | 0.344 | 0.607 | 1.77× |
| 1 | 10 | 0.555 | 1.003 | 1.81× |
| 10 | 1 | 0.052 | 0.091 | 1.76× |
| 10 | 5 | 0.254 | 0.531 | 2.09× |
| 10 | 10 | 0.505 | 0.986 | 1.95× |
| 100 | 1 | 0.055 | 0.098 | 1.77× |
| 100 | 5 | 0.229 | 0.455 | 1.99× |
| 100 | 10 | 0.537 | 1.094 | 2.04× |
| 1,000 | 1 | 0.064 | 0.094 | 1.47× |
| 1,000 | 5 | 0.243 | 0.491 | 2.02× |
| 1,000 | 10 | 0.778 | 1.655 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
