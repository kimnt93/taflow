# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.22M | 0.039 | 25.40M | 0.030 | 0.61× | 0.76× |
| 10,000 | 0.295 | 33.89M | 0.281 | 35.61M | 0.088 | 0.30× | 0.31× |
| 100,000 | 2.717 | 36.80M | 2.685 | 37.24M | 0.644 | 0.24× | 0.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.127 | 1.40× |
| 1 | 5 | 0.381 | 0.455 | 1.19× |
| 1 | 10 | 0.648 | 0.880 | 1.36× |
| 10 | 1 | 0.075 | 0.088 | 1.18× |
| 10 | 5 | 0.303 | 0.428 | 1.41× |
| 10 | 10 | 0.641 | 0.878 | 1.37× |
| 100 | 1 | 0.074 | 0.088 | 1.19× |
| 100 | 5 | 0.308 | 0.418 | 1.36× |
| 100 | 10 | 0.684 | 0.930 | 1.36× |
| 1,000 | 1 | 0.100 | 0.101 | 1.01× |
| 1,000 | 5 | 0.313 | 0.465 | 1.49× |
| 1,000 | 10 | 0.675 | 0.969 | 1.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
