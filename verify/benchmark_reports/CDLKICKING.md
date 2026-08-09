# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 99.00M | 0.009 | 116.54M | 0.038 | 3.81× | 4.49× |
| 10,000 | 0.091 | 110.34M | 0.089 | 112.45M | 0.172 | 1.89× | 1.93× |
| 100,000 | 0.962 | 103.98M | 0.951 | 105.17M | 1.508 | 1.57× | 1.59× |
| 1,000,000 | 10.512 | 95.13M | 10.017 | 99.83M | 14.451 | 1.37× | 1.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.151 | 1.15× |
| 1 | 5 | 0.383 | 0.491 | 1.28× |
| 1 | 10 | 0.530 | 0.965 | 1.82× |
| 10 | 1 | 0.053 | 0.095 | 1.78× |
| 10 | 5 | 0.239 | 0.442 | 1.85× |
| 10 | 10 | 0.499 | 0.906 | 1.81× |
| 100 | 1 | 0.053 | 0.090 | 1.71× |
| 100 | 5 | 0.249 | 0.441 | 1.77× |
| 100 | 10 | 0.514 | 0.895 | 1.74× |
| 1,000 | 1 | 0.064 | 0.113 | 1.77× |
| 1,000 | 5 | 0.238 | 0.512 | 2.16× |
| 1,000 | 10 | 0.569 | 1.135 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
