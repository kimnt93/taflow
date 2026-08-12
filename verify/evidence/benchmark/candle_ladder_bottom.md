# CandleLadderBottom benchmark (`CDLLADDERBOTTOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.27M | 0.016 | 61.62M | 0.042 | 2.06× | 2.58× |
| 10,000 | 0.148 | 67.39M | 0.143 | 69.73M | 0.104 | 0.70× | 0.73× |
| 100,000 | 1.371 | 72.96M | 1.309 | 76.37M | 0.625 | 0.46× | 0.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.123 | 1.45× |
| 1 | 5 | 0.276 | 0.448 | 1.62× |
| 1 | 10 | 0.524 | 0.979 | 1.87× |
| 10 | 1 | 0.063 | 0.091 | 1.46× |
| 10 | 5 | 0.274 | 0.437 | 1.60× |
| 10 | 10 | 0.535 | 0.881 | 1.65× |
| 100 | 1 | 0.056 | 0.094 | 1.69× |
| 100 | 5 | 0.302 | 0.472 | 1.56× |
| 100 | 10 | 0.537 | 0.931 | 1.73× |
| 1,000 | 1 | 0.069 | 0.096 | 1.38× |
| 1,000 | 5 | 0.273 | 0.469 | 1.72× |
| 1,000 | 10 | 0.574 | 1.031 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
