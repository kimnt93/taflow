# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 140.59M | 0.004 | 265.21M | 0.032 | 4.50× | 8.50× |
| 10,000 | 0.059 | 168.20M | 0.054 | 183.75M | 0.087 | 1.47× | 1.60× |
| 100,000 | 0.743 | 134.61M | 0.711 | 140.69M | 0.603 | 0.81× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | 0.116 | 2.03× |
| 1 | 5 | 0.268 | 0.445 | 1.66× |
| 1 | 10 | 0.399 | 0.874 | 2.19× |
| 10 | 1 | 0.042 | 0.087 | 2.05× |
| 10 | 5 | 0.185 | 0.446 | 2.40× |
| 10 | 10 | 0.398 | 0.900 | 2.26× |
| 100 | 1 | 0.041 | 0.089 | 2.14× |
| 100 | 5 | 0.191 | 0.425 | 2.22× |
| 100 | 10 | 0.403 | 0.918 | 2.28× |
| 1,000 | 1 | 0.051 | 0.092 | 1.82× |
| 1,000 | 5 | 0.196 | 0.446 | 2.27× |
| 1,000 | 10 | 0.442 | 1.001 | 2.26× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
