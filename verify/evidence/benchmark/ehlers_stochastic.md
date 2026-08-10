# EhlersStochastic benchmark (`EhlersStochastic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.96M | 0.040 | 25.11M | 0.184 | 4.60× | 4.63× |
| 10,000 | 0.349 | 28.67M | 0.342 | 29.21M | 0.734 | 2.10× | 2.14× |
| 100,000 | 3.551 | 28.16M | 3.356 | 29.79M | 6.217 | 1.75× | 1.85× |
| 1,000,000 | 35.099 | 28.49M | 36.706 | 27.24M | 63.832 | 1.82× | 1.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.267 | 2.67× |
| 1 | 5 | 0.298 | 0.945 | 3.18× |
| 1 | 10 | 0.476 | 2.077 | 4.36× |
| 10 | 1 | 0.050 | 0.194 | 3.85× |
| 10 | 5 | 0.213 | 0.956 | 4.49× |
| 10 | 10 | 0.484 | 2.117 | 4.37× |
| 100 | 1 | 0.053 | 0.198 | 3.71× |
| 100 | 5 | 0.595 | 1.002 | 1.68× |
| 100 | 10 | 0.513 | 2.140 | 4.17× |
| 1,000 | 1 | 0.088 | 0.257 | 2.91× |
| 1,000 | 5 | 0.240 | 1.286 | 5.37× |
| 1,000 | 10 | 0.527 | 2.779 | 5.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
