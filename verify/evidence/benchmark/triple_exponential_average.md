# TripleExponentialAverage benchmark (`T3` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.81M | 0.005 | 210.50M | 0.038 | 6.78× | 8.03× |
| 10,000 | 0.039 | 253.41M | 0.035 | 283.03M | 0.075 | 1.90× | 2.13× |
| 100,000 | 0.372 | 268.59M | 0.351 | 285.20M | 0.443 | 1.19× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.126 | 1.99× |
| 1 | 5 | 0.264 | 0.491 | 1.86× |
| 1 | 10 | 0.382 | 0.965 | 2.53× |
| 10 | 1 | 0.044 | 0.099 | 2.26× |
| 10 | 5 | 0.201 | 0.501 | 2.49× |
| 10 | 10 | 0.430 | 0.972 | 2.26× |
| 100 | 1 | 0.046 | 0.096 | 2.08× |
| 100 | 5 | 0.179 | 0.458 | 2.56× |
| 100 | 10 | 0.400 | 1.009 | 2.53× |
| 1,000 | 1 | 0.066 | 0.100 | 1.52× |
| 1,000 | 5 | 0.210 | 0.470 | 2.24× |
| 1,000 | 10 | 0.423 | 1.020 | 2.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
