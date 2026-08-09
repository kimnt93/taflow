# MathAbs benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 225.66M | 0.003 | 299.20M | 0.002 | 0.41× | 0.54× |
| 10,000 | 0.017 | 595.76M | 0.014 | 740.36M | 0.004 | 0.23× | 0.29× |
| 100,000 | 0.142 | 704.81M | 0.115 | 873.20M | 0.032 | 0.23× | 0.28× |
| 1,000,000 | 3.052 | 327.63M | 2.434 | 410.84M | 0.590 | 0.19× | 0.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.095 | 0.69× |
| 1 | 5 | 0.327 | 0.203 | 0.62× |
| 1 | 10 | 0.461 | 0.387 | 0.84× |
| 10 | 1 | 0.053 | 0.045 | 0.85× |
| 10 | 5 | 0.226 | 0.187 | 0.83× |
| 10 | 10 | 0.463 | 0.383 | 0.83× |
| 100 | 1 | 0.052 | 0.043 | 0.82× |
| 100 | 5 | 0.224 | 0.193 | 0.86× |
| 100 | 10 | 0.493 | 0.414 | 0.84× |
| 1,000 | 1 | 0.050 | 0.043 | 0.86× |
| 1,000 | 5 | 0.215 | 0.186 | 0.86× |
| 1,000 | 10 | 0.478 | 0.428 | 0.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
