# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.45M | 0.076 | 13.08M | 0.033 | 0.38× | 0.43× |
| 10,000 | 0.695 | 14.39M | 0.694 | 14.40M | 0.084 | 0.12× | 0.12× |
| 100,000 | 7.178 | 13.93M | 6.463 | 15.47M | 0.596 | 0.08× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.151 | 0.169 | 1.12× |
| 1 | 5 | 0.408 | 0.450 | 1.10× |
| 1 | 10 | 0.644 | 0.939 | 1.46× |
| 10 | 1 | 0.067 | 0.106 | 1.59× |
| 10 | 5 | 0.319 | 0.428 | 1.34× |
| 10 | 10 | 0.653 | 0.906 | 1.39× |
| 100 | 1 | 0.078 | 0.086 | 1.09× |
| 100 | 5 | 0.317 | 0.432 | 1.36× |
| 100 | 10 | 0.640 | 0.926 | 1.45× |
| 1,000 | 1 | 0.151 | 0.102 | 0.68× |
| 1,000 | 5 | 0.347 | 0.483 | 1.39× |
| 1,000 | 10 | 0.681 | 0.991 | 1.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
