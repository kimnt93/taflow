# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.50M | 0.006 | 172.70M | 0.033 | 3.83× | 5.78× |
| 10,000 | 0.121 | 82.65M | 0.113 | 88.62M | 0.112 | 0.93× | 1.00× |
| 100,000 | 1.301 | 76.85M | 1.258 | 79.49M | 0.879 | 0.68× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.104 | 1.01× |
| 1 | 5 | 0.319 | 0.469 | 1.47× |
| 1 | 10 | 0.396 | 0.908 | 2.29× |
| 10 | 1 | 0.041 | 0.092 | 2.24× |
| 10 | 5 | 0.185 | 0.424 | 2.30× |
| 10 | 10 | 0.355 | 0.894 | 2.52× |
| 100 | 1 | 0.041 | 0.089 | 2.18× |
| 100 | 5 | 0.190 | 0.419 | 2.20× |
| 100 | 10 | 0.379 | 0.887 | 2.34× |
| 1,000 | 1 | 0.060 | 0.095 | 1.58× |
| 1,000 | 5 | 0.194 | 0.470 | 2.42× |
| 1,000 | 10 | 0.429 | 1.005 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
