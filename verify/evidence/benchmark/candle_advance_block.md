# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.17M | 0.017 | 58.88M | 0.047 | 2.42× | 2.79× |
| 10,000 | 0.193 | 51.82M | 0.197 | 50.65M | 0.214 | 1.11× | 1.08× |
| 100,000 | 2.069 | 48.33M | 1.963 | 50.93M | 1.877 | 0.91× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.146 | 0.178 | 1.21× |
| 1 | 5 | 0.235 | 0.460 | 1.96× |
| 1 | 10 | 0.377 | 0.917 | 2.43× |
| 10 | 1 | 0.042 | 0.088 | 2.11× |
| 10 | 5 | 0.184 | 0.466 | 2.53× |
| 10 | 10 | 0.422 | 0.910 | 2.15× |
| 100 | 1 | 0.042 | 0.091 | 2.18× |
| 100 | 5 | 0.192 | 0.439 | 2.29× |
| 100 | 10 | 0.409 | 0.949 | 2.32× |
| 1,000 | 1 | 0.065 | 0.108 | 1.65× |
| 1,000 | 5 | 0.214 | 0.513 | 2.40× |
| 1,000 | 10 | 0.427 | 1.114 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
