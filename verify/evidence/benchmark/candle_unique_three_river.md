# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.39M | 0.017 | 59.19M | 0.033 | 1.70× | 1.96× |
| 10,000 | 0.146 | 68.32M | 0.151 | 66.27M | 0.097 | 0.66× | 0.64× |
| 100,000 | 1.470 | 68.01M | 1.404 | 71.20M | 0.620 | 0.42× | 0.44× |
| 1,000,000 | 14.101 | 70.92M | 13.878 | 72.05M | 6.143 | 0.44× | 0.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.137 | 1.15× |
| 1 | 5 | 0.317 | 0.578 | 1.82× |
| 1 | 10 | 0.672 | 0.956 | 1.42× |
| 10 | 1 | 0.055 | 0.090 | 1.66× |
| 10 | 5 | 0.247 | 0.426 | 1.72× |
| 10 | 10 | 0.591 | 0.978 | 1.66× |
| 100 | 1 | 0.057 | 0.095 | 1.66× |
| 100 | 5 | 0.247 | 0.423 | 1.71× |
| 100 | 10 | 0.544 | 1.017 | 1.87× |
| 1,000 | 1 | 0.081 | 0.107 | 1.32× |
| 1,000 | 5 | 0.299 | 0.478 | 1.60× |
| 1,000 | 10 | 0.540 | 0.983 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
