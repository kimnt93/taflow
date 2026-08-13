# RollingVarianceRatio benchmark (`VarianceRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 2.361 | 423.54K | 2.355 | 424.54K | 0.353 | 0.15× | 0.15× |
| 10,000 | 25.604 | 390.56K | 25.627 | 390.22K | 2.215 | 0.09× | 0.09× |
| 100,000 | 257.926 | 387.71K | 255.374 | 391.58K | 20.679 | 0.08× | 0.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.174 | 0.279 | 1.60× |
| 1 | 5 | 0.378 | 1.357 | 3.59× |
| 1 | 10 | 0.602 | 2.472 | 4.11× |
| 10 | 1 | 0.072 | 0.229 | 3.17× |
| 10 | 5 | 0.304 | 1.366 | 4.49× |
| 10 | 10 | 0.614 | 2.604 | 4.24× |
| 100 | 1 | 0.208 | 0.253 | 1.21× |
| 100 | 5 | 0.374 | 1.411 | 3.77× |
| 100 | 10 | 0.720 | 2.579 | 3.58× |
| 1,000 | 1 | 2.643 | 0.487 | 0.18× |
| 1,000 | 5 | 4.254 | 2.824 | 0.66× |
| 1,000 | 10 | 5.256 | 5.136 | 0.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
