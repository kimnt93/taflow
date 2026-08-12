# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 158.55M | 0.005 | 185.04M | 0.037 | 5.82× | 6.80× |
| 10,000 | 0.035 | 285.47M | 0.035 | 285.48M | 0.071 | 2.03× | 2.03× |
| 100,000 | 0.362 | 276.07M | 0.320 | 312.46M | 0.328 | 0.91× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.115 | 1.58× |
| 1 | 5 | 0.315 | 0.465 | 1.48× |
| 1 | 10 | 0.469 | 0.927 | 1.98× |
| 10 | 1 | 0.047 | 0.097 | 2.07× |
| 10 | 5 | 0.262 | 0.491 | 1.87× |
| 10 | 10 | 0.474 | 0.956 | 2.02× |
| 100 | 1 | 0.052 | 0.094 | 1.81× |
| 100 | 5 | 0.229 | 0.479 | 2.09× |
| 100 | 10 | 0.563 | 0.961 | 1.71× |
| 1,000 | 1 | 0.054 | 0.095 | 1.75× |
| 1,000 | 5 | 0.238 | 0.489 | 2.06× |
| 1,000 | 10 | 0.500 | 1.049 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
