# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 57.68M | 0.015 | 65.94M | 0.054 | 3.11× | 3.56× |
| 10,000 | 0.147 | 67.88M | 0.144 | 69.58M | 0.165 | 1.12× | 1.15× |
| 100,000 | 1.390 | 71.96M | 1.372 | 72.89M | 1.206 | 0.87× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.156 | 1.42× |
| 1 | 5 | 0.314 | 0.548 | 1.74× |
| 1 | 10 | 0.530 | 1.137 | 2.14× |
| 10 | 1 | 0.073 | 0.118 | 1.63× |
| 10 | 5 | 0.244 | 0.533 | 2.18× |
| 10 | 10 | 0.512 | 1.135 | 2.22× |
| 100 | 1 | 0.056 | 0.107 | 1.91× |
| 100 | 5 | 0.293 | 0.573 | 1.96× |
| 100 | 10 | 0.571 | 1.080 | 1.89× |
| 1,000 | 1 | 0.068 | 0.120 | 1.78× |
| 1,000 | 5 | 0.268 | 0.617 | 2.30× |
| 1,000 | 10 | 0.606 | 1.288 | 2.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
