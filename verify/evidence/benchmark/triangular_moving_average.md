# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 178.17M | 0.005 | 216.56M | 0.034 | 6.05× | 7.35× |
| 10,000 | 0.042 | 237.31M | 0.039 | 254.27M | 0.060 | 1.41× | 1.52× |
| 100,000 | 0.386 | 259.08M | 0.370 | 270.40M | 0.311 | 0.81× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.104 | 1.07× |
| 1 | 5 | 0.364 | 0.511 | 1.40× |
| 1 | 10 | 0.377 | 0.901 | 2.39× |
| 10 | 1 | 0.042 | 0.097 | 2.33× |
| 10 | 5 | 0.189 | 0.448 | 2.37× |
| 10 | 10 | 0.367 | 0.999 | 2.72× |
| 100 | 1 | 0.048 | 0.100 | 2.08× |
| 100 | 5 | 0.226 | 0.506 | 2.24× |
| 100 | 10 | 0.492 | 0.994 | 2.02× |
| 1,000 | 1 | 0.046 | 0.095 | 2.07× |
| 1,000 | 5 | 0.216 | 0.468 | 2.17× |
| 1,000 | 10 | 0.421 | 0.946 | 2.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
