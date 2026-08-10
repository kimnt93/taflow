# CandleThreeBlackCrows benchmark (`CDL3BLACKCROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.19M | 0.010 | 101.87M | 0.041 | 3.16× | 4.17× |
| 10,000 | 0.068 | 146.43M | 0.064 | 155.72M | 0.097 | 1.42× | 1.51× |
| 100,000 | 0.855 | 116.93M | 0.788 | 126.83M | 0.691 | 0.81× | 0.88× |
| 1,000,000 | 8.220 | 121.66M | 7.806 | 128.10M | 6.585 | 0.80× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.120 | 1.47× |
| 1 | 5 | 0.401 | 0.514 | 1.28× |
| 1 | 10 | 0.528 | 0.912 | 1.73× |
| 10 | 1 | 0.054 | 0.090 | 1.66× |
| 10 | 5 | 0.257 | 0.480 | 1.87× |
| 10 | 10 | 0.598 | 0.941 | 1.57× |
| 100 | 1 | 0.053 | 0.085 | 1.61× |
| 100 | 5 | 0.259 | 0.426 | 1.65× |
| 100 | 10 | 0.562 | 1.085 | 1.93× |
| 1,000 | 1 | 0.081 | 0.108 | 1.33× |
| 1,000 | 5 | 0.265 | 0.469 | 1.77× |
| 1,000 | 10 | 0.580 | 1.068 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
