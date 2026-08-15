# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.57M | 0.044 | 22.94M | 0.071 | 1.60× | 1.63× |
| 10,000 | 0.441 | 22.65M | 0.441 | 22.69M | 0.486 | 1.10× | 1.10× |
| 100,000 | 4.262 | 23.46M | 4.403 | 22.71M | 4.321 | 1.01× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.107 | 0.78× |
| 1 | 5 | 0.261 | 0.440 | 1.68× |
| 1 | 10 | 0.358 | 0.884 | 2.47× |
| 10 | 1 | 0.045 | 0.084 | 1.85× |
| 10 | 5 | 0.197 | 0.424 | 2.16× |
| 10 | 10 | 0.395 | 0.909 | 2.30× |
| 100 | 1 | 0.045 | 0.092 | 2.06× |
| 100 | 5 | 0.185 | 0.416 | 2.25× |
| 100 | 10 | 0.429 | 0.942 | 2.20× |
| 1,000 | 1 | 0.086 | 0.131 | 1.53× |
| 1,000 | 5 | 0.194 | 0.648 | 3.34× |
| 1,000 | 10 | 0.418 | 1.383 | 3.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
