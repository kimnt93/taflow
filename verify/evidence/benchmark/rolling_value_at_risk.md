# RollingValueAtRisk benchmark (`ValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.132 | 7.56M | 0.124 | 8.04M | 0.350 | 2.65× | 2.82× |
| 10,000 | 1.290 | 7.75M | 1.355 | 7.38M | 2.948 | 2.29× | 2.18× |
| 100,000 | 13.527 | 7.39M | 13.018 | 7.68M | 16.338 | 1.21× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.126 | 0.289 | 2.29× |
| 1 | 5 | 0.304 | 1.225 | 4.03× |
| 1 | 10 | 0.481 | 2.796 | 5.82× |
| 10 | 1 | 0.065 | 0.246 | 3.78× |
| 10 | 5 | 0.278 | 1.198 | 4.31× |
| 10 | 10 | 0.511 | 2.684 | 5.25× |
| 100 | 1 | 0.064 | 0.257 | 3.99× |
| 100 | 5 | 0.261 | 1.491 | 5.72× |
| 100 | 10 | 0.567 | 2.781 | 4.90× |
| 1,000 | 1 | 0.193 | 0.414 | 2.15× |
| 1,000 | 5 | 0.394 | 2.334 | 5.92× |
| 1,000 | 10 | 0.622 | 4.524 | 7.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
