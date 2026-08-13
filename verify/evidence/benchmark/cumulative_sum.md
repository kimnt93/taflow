# CumulativeSum benchmark (`numpy.cumsum` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.71M | 0.016 | 64.25M | 0.017 | 0.88× | 1.11× |
| 10,000 | 0.102 | 98.21M | 0.146 | 68.43M | 0.034 | 0.33× | 0.23× |
| 100,000 | 0.906 | 110.38M | 0.876 | 114.17M | 0.220 | 0.24× | 0.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.123 | 1.04× |
| 1 | 5 | 0.397 | 0.343 | 0.86× |
| 1 | 10 | 0.568 | 0.620 | 1.09× |
| 10 | 1 | 0.064 | 0.064 | 0.99× |
| 10 | 5 | 0.265 | 0.289 | 1.09× |
| 10 | 10 | 0.572 | 0.625 | 1.09× |
| 100 | 1 | 0.061 | 0.062 | 1.02× |
| 100 | 5 | 0.274 | 0.302 | 1.10× |
| 100 | 10 | 0.574 | 0.627 | 1.09× |
| 1,000 | 1 | 0.070 | 0.064 | 0.91× |
| 1,000 | 5 | 0.281 | 0.345 | 1.23× |
| 1,000 | 10 | 0.608 | 0.696 | 1.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
