# MedianChannel benchmark (`MedianChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.173 | 5.77M | 0.173 | 5.78M | 0.959 | 5.53× | 5.54× |
| 10,000 | 1.704 | 5.87M | 1.742 | 5.74M | 7.810 | 4.58× | 4.48× |
| 100,000 | 17.342 | 5.77M | 17.585 | 5.69M | 78.362 | 4.52× | 4.46× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.291 | 3.72× |
| 1 | 5 | 0.212 | 1.350 | 6.35× |
| 1 | 10 | 0.388 | 2.600 | 6.70× |
| 10 | 1 | 0.049 | 0.250 | 5.13× |
| 10 | 5 | 0.205 | 1.417 | 6.91× |
| 10 | 10 | 0.442 | 2.734 | 6.18× |
| 100 | 1 | 0.071 | 0.320 | 4.51× |
| 100 | 5 | 0.238 | 1.886 | 7.92× |
| 100 | 10 | 0.454 | 3.343 | 7.36× |
| 1,000 | 1 | 0.218 | 1.234 | 5.65× |
| 1,000 | 5 | 0.327 | 5.410 | 16.52× |
| 1,000 | 10 | 0.610 | 11.297 | 18.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
