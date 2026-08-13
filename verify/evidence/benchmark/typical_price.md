# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.61M | 0.023 | 42.68M | 0.030 | 0.99× | 1.26× |
| 10,000 | 0.159 | 63.06M | 0.150 | 66.87M | 0.033 | 0.21× | 0.22× |
| 100,000 | 1.444 | 69.27M | 1.414 | 70.74M | 0.083 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.172 | 0.123 | 0.71× |
| 1 | 5 | 0.453 | 0.482 | 1.07× |
| 1 | 10 | 0.759 | 0.941 | 1.24× |
| 10 | 1 | 0.063 | 0.089 | 1.41× |
| 10 | 5 | 0.290 | 0.414 | 1.43× |
| 10 | 10 | 0.607 | 0.897 | 1.48× |
| 100 | 1 | 0.064 | 0.090 | 1.40× |
| 100 | 5 | 0.292 | 0.420 | 1.44× |
| 100 | 10 | 0.601 | 0.905 | 1.51× |
| 1,000 | 1 | 0.082 | 0.098 | 1.19× |
| 1,000 | 5 | 0.302 | 0.424 | 1.40× |
| 1,000 | 10 | 0.636 | 0.885 | 1.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
