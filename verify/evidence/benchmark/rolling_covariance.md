# RollingCovariance benchmark (`RollingCovariance` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.66M | 0.012 | 80.62M | 0.204 | 15.02× | 16.44× |
| 10,000 | 0.107 | 93.50M | 0.096 | 104.38M | 0.814 | 7.61× | 8.50× |
| 100,000 | 0.958 | 104.41M | 0.904 | 110.68M | 6.604 | 6.90× | 7.31× |
| 1,000,000 | 9.508 | 105.18M | 9.500 | 105.27M | 64.960 | 6.83× | 6.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.158 | 0.359 | 2.27× |
| 1 | 5 | 0.358 | 1.259 | 3.51× |
| 1 | 10 | 0.497 | 2.301 | 4.63× |
| 10 | 1 | 0.050 | 0.211 | 4.26× |
| 10 | 5 | 0.242 | 1.227 | 5.07× |
| 10 | 10 | 0.492 | 2.344 | 4.77× |
| 100 | 1 | 0.055 | 0.232 | 4.18× |
| 100 | 5 | 0.263 | 1.285 | 4.90× |
| 100 | 10 | 0.505 | 2.370 | 4.69× |
| 1,000 | 1 | 0.059 | 0.278 | 4.67× |
| 1,000 | 5 | 0.224 | 1.555 | 6.95× |
| 1,000 | 10 | 0.516 | 2.923 | 5.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
