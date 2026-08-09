# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 119.59M | 0.007 | 144.14M | 0.036 | 4.35× | 5.24× |
| 10,000 | 0.046 | 216.49M | 0.045 | 223.25M | 0.062 | 1.34× | 1.39× |
| 100,000 | 0.461 | 216.99M | 0.429 | 233.19M | 0.348 | 0.76× | 0.81× |
| 1,000,000 | 4.933 | 202.70M | 4.509 | 221.79M | 3.228 | 0.65× | 0.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.124 | 1.40× |
| 1 | 5 | 0.286 | 0.546 | 1.91× |
| 1 | 10 | 0.576 | 1.153 | 2.00× |
| 10 | 1 | 0.058 | 0.110 | 1.88× |
| 10 | 5 | 0.265 | 0.550 | 2.07× |
| 10 | 10 | 0.555 | 1.157 | 2.09× |
| 100 | 1 | 0.054 | 0.095 | 1.78× |
| 100 | 5 | 0.272 | 0.505 | 1.86× |
| 100 | 10 | 0.747 | 1.201 | 1.61× |
| 1,000 | 1 | 0.062 | 0.110 | 1.77× |
| 1,000 | 5 | 0.273 | 0.536 | 1.96× |
| 1,000 | 10 | 0.619 | 1.178 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
