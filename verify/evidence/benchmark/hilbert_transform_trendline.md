# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.080 | 12.56M | 0.082 | 12.25M | 0.084 | 1.06× | 1.03× |
| 10,000 | 0.721 | 13.87M | 0.696 | 14.38M | 0.599 | 0.83× | 0.86× |
| 100,000 | 7.030 | 14.22M | 8.313 | 12.03M | 7.701 | 1.10× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.134 | 0.098 | 0.73× |
| 1 | 5 | 0.412 | 0.499 | 1.21× |
| 1 | 10 | 0.532 | 1.210 | 2.27× |
| 10 | 1 | 0.082 | 0.134 | 1.64× |
| 10 | 5 | 0.346 | 0.644 | 1.86× |
| 10 | 10 | 0.564 | 0.962 | 1.71× |
| 100 | 1 | 0.070 | 0.091 | 1.31× |
| 100 | 5 | 0.239 | 0.457 | 1.92× |
| 100 | 10 | 0.496 | 1.050 | 2.12× |
| 1,000 | 1 | 0.129 | 0.163 | 1.27× |
| 1,000 | 5 | 0.253 | 0.729 | 2.88× |
| 1,000 | 10 | 0.554 | 1.621 | 2.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
