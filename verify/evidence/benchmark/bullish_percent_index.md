# BullishPercentIndex benchmark (`BullishPercentIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.24M | 0.006 | 173.21M | 11.738 | 1622.75× | 2033.23× |
| 10,000 | 0.031 | 327.66M | 0.028 | 355.03M | 116.922 | 3831.00× | 4151.03× |
| 100,000 | 0.246 | 406.35M | 0.250 | 399.74M | 1156.899 | 4701.05× | 4624.64× |
| 1,000,000 | 3.094 | 323.24M | 2.660 | 375.92M | 11490.692 | 3714.22× | 4319.55× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.293 | 2.66× |
| 1 | 5 | 0.292 | 1.333 | 4.56× |
| 1 | 10 | 0.465 | 2.274 | 4.89× |
| 10 | 1 | 0.056 | 0.332 | 5.97× |
| 10 | 5 | 0.238 | 1.551 | 6.53× |
| 10 | 10 | 0.477 | 3.338 | 7.00× |
| 100 | 1 | 0.050 | 1.368 | 27.45× |
| 100 | 5 | 0.249 | 7.346 | 29.52× |
| 100 | 10 | 0.513 | 14.560 | 28.39× |
| 1,000 | 1 | 0.054 | 11.621 | 213.39× |
| 1,000 | 5 | 0.289 | 59.602 | 206.15× |
| 1,000 | 10 | 0.710 | 115.870 | 163.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
