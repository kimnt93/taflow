# CandleTriStar benchmark (`CDLTRISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 57.06M | 0.016 | 63.10M | 0.033 | 1.90× | 2.10× |
| 10,000 | 0.109 | 91.82M | 0.101 | 98.54M | 0.096 | 0.88× | 0.95× |
| 100,000 | 1.048 | 95.38M | 1.029 | 97.17M | 0.677 | 0.65× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.126 | 1.06× |
| 1 | 5 | 0.318 | 0.492 | 1.55× |
| 1 | 10 | 0.562 | 1.132 | 2.01× |
| 10 | 1 | 0.070 | 0.107 | 1.52× |
| 10 | 5 | 0.298 | 0.516 | 1.73× |
| 10 | 10 | 0.982 | 1.198 | 1.22× |
| 100 | 1 | 0.090 | 0.122 | 1.35× |
| 100 | 5 | 0.394 | 0.582 | 1.47× |
| 100 | 10 | 0.583 | 1.087 | 1.86× |
| 1,000 | 1 | 0.102 | 0.137 | 1.35× |
| 1,000 | 5 | 0.293 | 0.507 | 1.73× |
| 1,000 | 10 | 0.606 | 1.074 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
