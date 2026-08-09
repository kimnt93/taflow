# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.04M | 0.017 | 57.55M | 0.046 | 2.42× | 2.62× |
| 10,000 | 0.137 | 73.03M | 0.135 | 74.26M | 0.164 | 1.19× | 1.21× |
| 100,000 | 1.414 | 70.70M | 1.345 | 74.36M | 1.341 | 0.95× | 1.00× |
| 1,000,000 | 14.361 | 69.63M | 13.868 | 72.11M | 13.201 | 0.92× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.154 | 1.43× |
| 1 | 5 | 0.373 | 0.541 | 1.45× |
| 1 | 10 | 0.500 | 1.000 | 2.00× |
| 10 | 1 | 0.049 | 0.096 | 1.98× |
| 10 | 5 | 0.221 | 0.441 | 2.00× |
| 10 | 10 | 0.519 | 0.998 | 1.92× |
| 100 | 1 | 0.053 | 0.099 | 1.86× |
| 100 | 5 | 0.247 | 0.463 | 1.87× |
| 100 | 10 | 0.517 | 1.031 | 1.99× |
| 1,000 | 1 | 0.069 | 0.114 | 1.66× |
| 1,000 | 5 | 0.262 | 0.543 | 2.07× |
| 1,000 | 10 | 0.514 | 1.142 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
