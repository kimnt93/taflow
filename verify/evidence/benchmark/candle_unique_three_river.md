# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.79M | 0.016 | 64.39M | 0.030 | 1.61× | 1.93× |
| 10,000 | 0.135 | 74.16M | 0.132 | 75.97M | 0.080 | 0.59× | 0.61× |
| 100,000 | 1.316 | 76.01M | 1.346 | 74.31M | 0.573 | 0.44× | 0.43× |
| 1,000,000 | 13.589 | 73.59M | 13.586 | 73.60M | 5.866 | 0.43× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.145 | 1.55× |
| 1 | 5 | 0.287 | 0.459 | 1.60× |
| 1 | 10 | 0.522 | 0.912 | 1.75× |
| 10 | 1 | 0.054 | 0.091 | 1.69× |
| 10 | 5 | 0.248 | 0.428 | 1.73× |
| 10 | 10 | 0.548 | 0.912 | 1.66× |
| 100 | 1 | 0.060 | 0.091 | 1.53× |
| 100 | 5 | 0.252 | 0.451 | 1.79× |
| 100 | 10 | 0.580 | 0.907 | 1.56× |
| 1,000 | 1 | 0.077 | 0.098 | 1.28× |
| 1,000 | 5 | 0.260 | 0.461 | 1.78× |
| 1,000 | 10 | 0.606 | 0.972 | 1.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
