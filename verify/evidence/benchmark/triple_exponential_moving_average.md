# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.44M | 0.012 | 86.59M | 0.043 | 3.50× | 3.72× |
| 10,000 | 0.101 | 99.45M | 0.096 | 103.81M | 0.118 | 1.17× | 1.22× |
| 100,000 | 0.972 | 102.87M | 0.910 | 109.89M | 0.885 | 0.91× | 0.97× |
| 1,000,000 | 9.656 | 103.56M | 9.783 | 102.21M | 11.327 | 1.17× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.157 | 1.72× |
| 1 | 5 | 0.340 | 0.714 | 2.10× |
| 1 | 10 | 0.581 | 1.030 | 1.77× |
| 10 | 1 | 0.055 | 0.102 | 1.85× |
| 10 | 5 | 0.254 | 0.511 | 2.01× |
| 10 | 10 | 0.564 | 1.169 | 2.07× |
| 100 | 1 | 0.059 | 0.108 | 1.84× |
| 100 | 5 | 0.329 | 0.514 | 1.57× |
| 100 | 10 | 0.522 | 1.011 | 1.94× |
| 1,000 | 1 | 0.061 | 0.097 | 1.59× |
| 1,000 | 5 | 0.245 | 0.499 | 2.03× |
| 1,000 | 10 | 0.516 | 1.045 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
