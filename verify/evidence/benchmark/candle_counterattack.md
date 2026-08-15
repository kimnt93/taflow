# CandleCounterAttack benchmark (`CDLCOUNTERATTACK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.96M | 0.006 | 175.31M | 0.036 | 5.13× | 6.39× |
| 10,000 | 0.070 | 143.11M | 0.060 | 165.47M | 0.137 | 1.96× | 2.27× |
| 100,000 | 0.914 | 109.45M | 0.885 | 113.02M | 1.154 | 1.26× | 1.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.164 | 1.64× |
| 1 | 5 | 0.274 | 0.485 | 1.77× |
| 1 | 10 | 0.375 | 0.966 | 2.58× |
| 10 | 1 | 0.048 | 0.101 | 2.11× |
| 10 | 5 | 0.181 | 0.423 | 2.33× |
| 10 | 10 | 0.401 | 0.919 | 2.30× |
| 100 | 1 | 0.041 | 0.099 | 2.43× |
| 100 | 5 | 0.186 | 0.478 | 2.57× |
| 100 | 10 | 0.439 | 0.917 | 2.09× |
| 1,000 | 1 | 0.053 | 0.102 | 1.91× |
| 1,000 | 5 | 0.222 | 0.481 | 2.16× |
| 1,000 | 10 | 0.428 | 1.152 | 2.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
