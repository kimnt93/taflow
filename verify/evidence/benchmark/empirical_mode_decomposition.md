# EmpiricalModeDecomposition benchmark (`EmpiricalModeDecomposition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.352 | 2.84M | 0.339 | 2.95M | 0.217 | 0.62× | 0.64× |
| 10,000 | 3.402 | 2.94M | 3.362 | 2.97M | 0.806 | 0.24× | 0.24× |
| 100,000 | 33.481 | 2.99M | 33.481 | 2.99M | 6.864 | 0.21× | 0.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.252 | 2.57× |
| 1 | 5 | 0.387 | 1.400 | 3.62× |
| 1 | 10 | 0.644 | 2.608 | 4.05× |
| 10 | 1 | 0.072 | 0.235 | 3.26× |
| 10 | 5 | 0.314 | 1.325 | 4.22× |
| 10 | 10 | 0.628 | 2.442 | 3.89× |
| 100 | 1 | 0.103 | 0.242 | 2.35× |
| 100 | 5 | 0.309 | 1.372 | 4.44× |
| 100 | 10 | 0.661 | 2.691 | 4.07× |
| 1,000 | 1 | 0.428 | 0.311 | 0.73× |
| 1,000 | 5 | 0.608 | 1.723 | 2.83× |
| 1,000 | 10 | 1.162 | 3.385 | 2.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
