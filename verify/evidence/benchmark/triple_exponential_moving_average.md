# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.94M | 0.012 | 84.07M | 0.040 | 3.14× | 3.34× |
| 10,000 | 0.096 | 104.48M | 0.095 | 104.93M | 0.118 | 1.23× | 1.23× |
| 100,000 | 0.963 | 103.81M | 0.938 | 106.61M | 0.901 | 0.94× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.119 | 1.28× |
| 1 | 5 | 0.342 | 0.453 | 1.33× |
| 1 | 10 | 0.478 | 0.972 | 2.03× |
| 10 | 1 | 0.054 | 0.093 | 1.74× |
| 10 | 5 | 0.225 | 0.434 | 1.93× |
| 10 | 10 | 0.472 | 0.907 | 1.92× |
| 100 | 1 | 0.052 | 0.104 | 1.99× |
| 100 | 5 | 0.216 | 0.449 | 2.08× |
| 100 | 10 | 0.452 | 0.915 | 2.02× |
| 1,000 | 1 | 0.061 | 0.101 | 1.67× |
| 1,000 | 5 | 0.244 | 0.516 | 2.11× |
| 1,000 | 10 | 0.540 | 1.064 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
