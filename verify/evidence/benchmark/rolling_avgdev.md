# RollingAverageDeviation benchmark (`AVGDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.36M | 0.017 | 59.37M | 0.045 | 2.58× | 2.67× |
| 10,000 | 0.160 | 62.68M | 0.156 | 64.21M | 0.168 | 1.05× | 1.08× |
| 100,000 | 1.608 | 62.18M | 1.540 | 64.95M | 1.393 | 0.87× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.130 | 1.35× |
| 1 | 5 | 0.309 | 0.489 | 1.58× |
| 1 | 10 | 0.430 | 0.921 | 2.14× |
| 10 | 1 | 0.043 | 0.086 | 2.00× |
| 10 | 5 | 0.196 | 0.444 | 2.27× |
| 10 | 10 | 0.389 | 0.904 | 2.33× |
| 100 | 1 | 0.047 | 0.109 | 2.31× |
| 100 | 5 | 0.207 | 0.430 | 2.07× |
| 100 | 10 | 0.401 | 0.912 | 2.28× |
| 1,000 | 1 | 0.057 | 0.109 | 1.92× |
| 1,000 | 5 | 0.185 | 0.528 | 2.85× |
| 1,000 | 10 | 0.427 | 1.101 | 2.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
