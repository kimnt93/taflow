# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.096 | 10.41M | 0.094 | 10.59M | 0.441 | 4.59× | 4.66× |
| 10,000 | 1.018 | 9.83M | 1.035 | 9.67M | 5.595 | 5.50× | 5.41× |
| 100,000 | 12.375 | 8.08M | 10.715 | 9.33M | 42.452 | 3.43× | 3.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.131 | 1.19× |
| 1 | 5 | 0.274 | 0.459 | 1.67× |
| 1 | 10 | 0.396 | 0.889 | 2.24× |
| 10 | 1 | 0.040 | 0.085 | 2.13× |
| 10 | 5 | 0.173 | 0.398 | 2.30× |
| 10 | 10 | 0.401 | 0.915 | 2.28× |
| 100 | 1 | 0.052 | 0.111 | 2.15× |
| 100 | 5 | 0.197 | 0.560 | 2.85× |
| 100 | 10 | 0.416 | 1.157 | 2.78× |
| 1,000 | 1 | 0.146 | 0.532 | 3.65× |
| 1,000 | 5 | 0.256 | 2.637 | 10.32× |
| 1,000 | 10 | 0.477 | 5.251 | 11.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
