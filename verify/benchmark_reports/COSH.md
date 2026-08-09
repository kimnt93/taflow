# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.74M | 0.009 | 107.40M | 0.036 | 3.40× | 3.85× |
| 10,000 | 0.071 | 141.40M | 0.067 | 148.70M | 0.085 | 1.20× | 1.26× |
| 100,000 | 0.745 | 134.24M | 0.655 | 152.74M | 0.624 | 0.84× | 0.95× |
| 1,000,000 | 7.617 | 131.29M | 7.293 | 137.11M | 6.282 | 0.82× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.106 | 1.07× |
| 1 | 5 | 0.306 | 0.525 | 1.72× |
| 1 | 10 | 0.555 | 1.038 | 1.87× |
| 10 | 1 | 0.056 | 0.099 | 1.78× |
| 10 | 5 | 0.262 | 0.477 | 1.82× |
| 10 | 10 | 0.544 | 1.042 | 1.92× |
| 100 | 1 | 0.058 | 0.101 | 1.76× |
| 100 | 5 | 0.287 | 0.482 | 1.68× |
| 100 | 10 | 0.543 | 1.008 | 1.86× |
| 1,000 | 1 | 0.060 | 0.101 | 1.70× |
| 1,000 | 5 | 0.261 | 0.510 | 1.95× |
| 1,000 | 10 | 0.548 | 1.113 | 2.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
