# OvernightIntradayReturn benchmark (`OvernightIntradayReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.31M | 0.020 | 50.36M | 0.639 | 21.28× | 32.18× |
| 10,000 | 0.078 | 128.39M | 0.076 | 131.01M | 6.244 | 80.17× | 81.81× |
| 100,000 | 0.709 | 140.99M | 0.607 | 164.67M | 50.196 | 70.77× | 82.66× |
| 1,000,000 | 7.759 | 128.88M | 7.051 | 141.83M | 573.586 | 73.92× | 81.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.299 | 2.85× |
| 1 | 5 | 0.442 | 1.222 | 2.77× |
| 1 | 10 | 0.568 | 2.477 | 4.36× |
| 10 | 1 | 0.059 | 0.246 | 4.17× |
| 10 | 5 | 0.276 | 1.359 | 4.93× |
| 10 | 10 | 0.584 | 2.550 | 4.36× |
| 100 | 1 | 0.059 | 0.291 | 4.91× |
| 100 | 5 | 0.287 | 1.649 | 5.75× |
| 100 | 10 | 0.588 | 3.065 | 5.21× |
| 1,000 | 1 | 0.073 | 0.912 | 12.49× |
| 1,000 | 5 | 0.296 | 4.273 | 14.46× |
| 1,000 | 10 | 0.603 | 8.268 | 13.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
