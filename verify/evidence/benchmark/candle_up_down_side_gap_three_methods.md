# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 219.54M | 0.003 | 349.06M | 0.032 | 6.93× | 11.02× |
| 10,000 | 0.022 | 464.59M | 0.018 | 547.43M | 0.085 | 3.94× | 4.64× |
| 100,000 | 0.199 | 502.53M | 0.179 | 559.18M | 0.607 | 3.05× | 3.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.167 | 1.60× |
| 1 | 5 | 0.281 | 0.441 | 1.57× |
| 1 | 10 | 0.388 | 1.010 | 2.60× |
| 10 | 1 | 0.043 | 0.095 | 2.21× |
| 10 | 5 | 0.182 | 0.465 | 2.55× |
| 10 | 10 | 0.429 | 0.959 | 2.24× |
| 100 | 1 | 0.045 | 0.086 | 1.91× |
| 100 | 5 | 0.189 | 0.487 | 2.58× |
| 100 | 10 | 0.409 | 0.984 | 2.40× |
| 1,000 | 1 | 0.042 | 0.098 | 2.32× |
| 1,000 | 5 | 0.187 | 0.471 | 2.52× |
| 1,000 | 10 | 0.468 | 1.027 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
