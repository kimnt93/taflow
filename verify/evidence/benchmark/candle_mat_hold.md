# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.155 | 6.46M | 0.146 | 6.85M | 0.038 | 0.25× | 0.26× |
| 10,000 | 1.333 | 7.50M | 1.389 | 7.20M | 0.122 | 0.09× | 0.09× |
| 100,000 | 12.944 | 7.73M | 13.195 | 7.58M | 0.861 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.147 | 1.62× |
| 1 | 5 | 0.347 | 0.489 | 1.41× |
| 1 | 10 | 0.633 | 0.985 | 1.56× |
| 10 | 1 | 0.069 | 0.099 | 1.44× |
| 10 | 5 | 0.305 | 0.453 | 1.49× |
| 10 | 10 | 0.656 | 0.988 | 1.51× |
| 100 | 1 | 0.086 | 0.096 | 1.12× |
| 100 | 5 | 0.309 | 0.499 | 1.61× |
| 100 | 10 | 0.696 | 0.998 | 1.43× |
| 1,000 | 1 | 0.211 | 0.115 | 0.55× |
| 1,000 | 5 | 0.372 | 0.506 | 1.36× |
| 1,000 | 10 | 0.718 | 1.059 | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
