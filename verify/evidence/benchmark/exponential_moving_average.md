# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 171.21M | 0.005 | 212.07M | 0.034 | 5.74× | 7.10× |
| 10,000 | 0.028 | 351.22M | 0.026 | 380.83M | 0.058 | 2.03× | 2.20× |
| 100,000 | 0.252 | 396.79M | 0.233 | 428.59M | 0.332 | 1.32× | 1.42× |
| 1,000,000 | 3.498 | 285.91M | 3.064 | 326.34M | 2.832 | 0.81× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.134 | 0.87× |
| 1 | 5 | 0.292 | 0.477 | 1.63× |
| 1 | 10 | 0.443 | 0.928 | 2.10× |
| 10 | 1 | 0.045 | 0.096 | 2.14× |
| 10 | 5 | 0.213 | 0.419 | 1.97× |
| 10 | 10 | 0.460 | 0.908 | 1.97× |
| 100 | 1 | 0.049 | 0.093 | 1.91× |
| 100 | 5 | 0.239 | 0.451 | 1.89× |
| 100 | 10 | 0.479 | 1.236 | 2.58× |
| 1,000 | 1 | 0.060 | 0.097 | 1.62× |
| 1,000 | 5 | 0.283 | 0.499 | 1.76× |
| 1,000 | 10 | 0.518 | 1.029 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
