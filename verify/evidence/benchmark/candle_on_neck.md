# CandleOnNeck benchmark (`CDLONNECK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.72M | 0.015 | 66.47M | 0.034 | 1.85× | 2.25× |
| 10,000 | 0.146 | 68.38M | 0.142 | 70.23M | 0.132 | 0.90× | 0.92× |
| 100,000 | 1.495 | 66.91M | 1.495 | 66.87M | 1.040 | 0.70× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.106 | 1.00× |
| 1 | 5 | 0.315 | 0.468 | 1.49× |
| 1 | 10 | 0.583 | 1.039 | 1.78× |
| 10 | 1 | 0.073 | 0.096 | 1.33× |
| 10 | 5 | 0.267 | 0.473 | 1.77× |
| 10 | 10 | 0.573 | 0.946 | 1.65× |
| 100 | 1 | 0.054 | 0.095 | 1.75× |
| 100 | 5 | 0.295 | 0.519 | 1.76× |
| 100 | 10 | 0.634 | 1.056 | 1.67× |
| 1,000 | 1 | 0.075 | 0.117 | 1.56× |
| 1,000 | 5 | 0.283 | 0.583 | 2.06× |
| 1,000 | 10 | 0.648 | 1.101 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
