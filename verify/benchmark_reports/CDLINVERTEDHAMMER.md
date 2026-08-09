# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.72M | 0.009 | 117.37M | 0.041 | 4.18× | 4.78× |
| 10,000 | 0.108 | 92.69M | 0.109 | 91.94M | 0.172 | 1.60× | 1.58× |
| 100,000 | 1.198 | 83.50M | 1.163 | 85.96M | 1.452 | 1.21× | 1.25× |
| 1,000,000 | 12.232 | 81.75M | 12.573 | 79.53M | 14.089 | 1.15× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.111 | 1.76× |
| 1 | 5 | 0.448 | 0.547 | 1.22× |
| 1 | 10 | 0.551 | 0.991 | 1.80× |
| 10 | 1 | 0.055 | 0.099 | 1.82× |
| 10 | 5 | 0.258 | 0.471 | 1.83× |
| 10 | 10 | 0.536 | 0.954 | 1.78× |
| 100 | 1 | 0.056 | 0.091 | 1.62× |
| 100 | 5 | 0.276 | 0.497 | 1.80× |
| 100 | 10 | 0.672 | 1.008 | 1.50× |
| 1,000 | 1 | 0.067 | 0.107 | 1.59× |
| 1,000 | 5 | 0.266 | 0.542 | 2.04× |
| 1,000 | 10 | 0.570 | 1.123 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
