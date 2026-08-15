# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.80M | 0.005 | 208.48M | 0.040 | 4.55× | 8.33× |
| 10,000 | 0.100 | 100.05M | 0.095 | 105.31M | 0.173 | 1.73× | 1.82× |
| 100,000 | 1.196 | 83.60M | 1.186 | 84.29M | 1.534 | 1.28× | 1.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.109 | 1.14× |
| 1 | 5 | 0.236 | 0.487 | 2.06× |
| 1 | 10 | 0.405 | 1.089 | 2.69× |
| 10 | 1 | 0.045 | 0.089 | 1.97× |
| 10 | 5 | 0.215 | 0.462 | 2.14× |
| 10 | 10 | 0.418 | 0.988 | 2.36× |
| 100 | 1 | 0.054 | 0.125 | 2.32× |
| 100 | 5 | 0.208 | 0.489 | 2.35× |
| 100 | 10 | 0.468 | 0.943 | 2.02× |
| 1,000 | 1 | 0.056 | 0.107 | 1.91× |
| 1,000 | 5 | 0.209 | 0.588 | 2.82× |
| 1,000 | 10 | 0.478 | 1.106 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
