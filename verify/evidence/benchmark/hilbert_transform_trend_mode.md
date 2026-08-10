# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.163 | 6.15M | 0.185 | 5.39M | 0.455 | 2.80× | 2.45× |
| 10,000 | 1.668 | 6.00M | 1.663 | 6.01M | 4.489 | 2.69× | 2.70× |
| 100,000 | 17.757 | 5.63M | 16.898 | 5.92M | 46.261 | 2.61× | 2.74× |
| 1,000,000 | 180.512 | 5.54M | 170.405 | 5.87M | 462.246 | 2.56× | 2.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.144 | 1.06× |
| 1 | 5 | 0.306 | 0.448 | 1.47× |
| 1 | 10 | 0.477 | 0.896 | 1.88× |
| 10 | 1 | 0.047 | 0.087 | 1.86× |
| 10 | 5 | 0.223 | 0.402 | 1.80× |
| 10 | 10 | 0.457 | 0.878 | 1.92× |
| 100 | 1 | 0.069 | 0.124 | 1.79× |
| 100 | 5 | 0.236 | 0.571 | 2.43× |
| 100 | 10 | 0.510 | 1.176 | 2.31× |
| 1,000 | 1 | 0.232 | 0.564 | 2.43× |
| 1,000 | 5 | 0.363 | 2.825 | 7.79× |
| 1,000 | 10 | 0.650 | 5.797 | 8.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
