# Crossover benchmark (`causal crossover` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 399.06M | 0.001 | 878.24M | 0.016 | 6.32× | 13.91× |
| 10,000 | 0.008 | 1.19G | 0.005 | 1.99G | 0.028 | 3.32× | 5.52× |
| 100,000 | 0.070 | 1.42G | 0.044 | 2.28G | 0.128 | 1.82× | 2.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.090 | 0.68× |
| 1 | 5 | 0.208 | 0.312 | 1.50× |
| 1 | 10 | 0.375 | 0.667 | 1.78× |
| 10 | 1 | 0.042 | 0.069 | 1.64× |
| 10 | 5 | 0.172 | 0.297 | 1.73× |
| 10 | 10 | 0.380 | 0.660 | 1.74× |
| 100 | 1 | 0.041 | 0.063 | 1.54× |
| 100 | 5 | 0.180 | 0.298 | 1.66× |
| 100 | 10 | 0.363 | 0.641 | 1.77× |
| 1,000 | 1 | 0.038 | 0.067 | 1.78× |
| 1,000 | 5 | 0.189 | 0.375 | 1.99× |
| 1,000 | 10 | 0.390 | 1.046 | 2.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
