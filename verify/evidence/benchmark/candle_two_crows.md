# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.84M | 0.009 | 108.58M | 0.033 | 2.43× | 3.62× |
| 10,000 | 0.083 | 120.43M | 0.075 | 134.01M | 0.125 | 1.51× | 1.68× |
| 100,000 | 1.019 | 98.12M | 1.037 | 96.48M | 1.127 | 1.11× | 1.09× |
| 1,000,000 | 10.439 | 95.79M | 10.884 | 91.88M | 10.324 | 0.99× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.154 | 1.41× |
| 1 | 5 | 0.339 | 0.492 | 1.45× |
| 1 | 10 | 0.621 | 0.994 | 1.60× |
| 10 | 1 | 0.073 | 0.121 | 1.66× |
| 10 | 5 | 0.303 | 0.578 | 1.91× |
| 10 | 10 | 0.689 | 1.051 | 1.53× |
| 100 | 1 | 0.063 | 0.088 | 1.40× |
| 100 | 5 | 0.300 | 0.493 | 1.64× |
| 100 | 10 | 0.704 | 1.013 | 1.44× |
| 1,000 | 1 | 0.075 | 0.108 | 1.44× |
| 1,000 | 5 | 0.327 | 0.579 | 1.77× |
| 1,000 | 10 | 0.717 | 1.157 | 1.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
