# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 177.66M | 0.005 | 186.86M | 0.038 | 6.73× | 7.08× |
| 10,000 | 0.025 | 393.74M | 0.022 | 454.81M | 0.055 | 2.17× | 2.51× |
| 100,000 | 0.224 | 446.89M | 0.214 | 468.17M | 0.265 | 1.18× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.124 | 1.41× |
| 1 | 5 | 0.301 | 0.562 | 1.87× |
| 1 | 10 | 0.498 | 1.040 | 2.09× |
| 10 | 1 | 0.052 | 0.095 | 1.84× |
| 10 | 5 | 0.217 | 0.502 | 2.32× |
| 10 | 10 | 0.538 | 0.972 | 1.81× |
| 100 | 1 | 0.051 | 0.102 | 2.00× |
| 100 | 5 | 0.247 | 0.469 | 1.90× |
| 100 | 10 | 0.537 | 1.071 | 1.99× |
| 1,000 | 1 | 0.055 | 0.106 | 1.92× |
| 1,000 | 5 | 0.237 | 0.484 | 2.04× |
| 1,000 | 10 | 0.493 | 1.060 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
