# ChaikinMoneyFlow benchmark (`ChaikinMoneyFlow` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.26M | 0.008 | 133.30M | 0.282 | 26.60× | 37.62× |
| 10,000 | 0.066 | 150.72M | 0.064 | 157.41M | 1.550 | 23.36× | 24.40× |
| 100,000 | 0.640 | 156.18M | 0.627 | 159.55M | 14.015 | 21.89× | 22.36× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.293 | 2.14× |
| 1 | 5 | 0.263 | 1.163 | 4.43× |
| 1 | 10 | 0.399 | 2.685 | 6.73× |
| 10 | 1 | 0.049 | 0.222 | 4.51× |
| 10 | 5 | 0.216 | 1.148 | 5.33× |
| 10 | 10 | 0.401 | 2.406 | 6.01× |
| 100 | 1 | 0.050 | 0.247 | 4.91× |
| 100 | 5 | 0.211 | 1.471 | 6.98× |
| 100 | 10 | 0.424 | 2.594 | 6.12× |
| 1,000 | 1 | 0.058 | 0.363 | 6.26× |
| 1,000 | 5 | 0.209 | 2.170 | 10.40× |
| 1,000 | 10 | 0.457 | 3.815 | 8.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
