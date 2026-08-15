# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 187.76M | 0.005 | 211.14M | 0.037 | 6.90× | 7.76× |
| 10,000 | 0.038 | 262.01M | 0.036 | 279.93M | 0.060 | 1.58× | 1.69× |
| 100,000 | 0.408 | 244.83M | 0.349 | 286.35M | 0.302 | 0.74× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.166 | 1.53× |
| 1 | 5 | 0.198 | 0.458 | 2.32× |
| 1 | 10 | 0.396 | 0.947 | 2.39× |
| 10 | 1 | 0.049 | 0.090 | 1.85× |
| 10 | 5 | 0.180 | 0.454 | 2.53× |
| 10 | 10 | 0.403 | 0.943 | 2.34× |
| 100 | 1 | 0.046 | 0.091 | 1.98× |
| 100 | 5 | 0.177 | 0.433 | 2.44× |
| 100 | 10 | 0.365 | 0.959 | 2.63× |
| 1,000 | 1 | 0.052 | 0.089 | 1.71× |
| 1,000 | 5 | 0.206 | 0.456 | 2.22× |
| 1,000 | 10 | 0.459 | 0.939 | 2.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
