# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 86.97M | 0.009 | 116.88M | 0.030 | 2.59× | 3.48× |
| 10,000 | 0.067 | 148.83M | 0.063 | 159.25M | 0.087 | 1.30× | 1.39× |
| 100,000 | 0.781 | 128.05M | 0.730 | 136.90M | 0.637 | 0.82× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.108 | 1.18× |
| 1 | 5 | 0.345 | 0.457 | 1.33× |
| 1 | 10 | 0.532 | 0.878 | 1.65× |
| 10 | 1 | 0.050 | 0.086 | 1.73× |
| 10 | 5 | 0.249 | 0.420 | 1.69× |
| 10 | 10 | 0.504 | 0.897 | 1.78× |
| 100 | 1 | 0.053 | 0.091 | 1.71× |
| 100 | 5 | 0.253 | 0.440 | 1.74× |
| 100 | 10 | 0.533 | 0.896 | 1.68× |
| 1,000 | 1 | 0.060 | 0.103 | 1.71× |
| 1,000 | 5 | 0.288 | 0.457 | 1.59× |
| 1,000 | 10 | 0.538 | 0.970 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
