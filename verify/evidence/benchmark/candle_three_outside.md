# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.86M | 0.003 | 324.41M | 0.029 | 4.51× | 9.39× |
| 10,000 | 0.028 | 360.05M | 0.023 | 434.35M | 0.089 | 3.21× | 3.87× |
| 100,000 | 0.236 | 424.41M | 0.223 | 447.92M | 0.563 | 2.39× | 2.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.114 | 0.97× |
| 1 | 5 | 0.292 | 0.544 | 1.86× |
| 1 | 10 | 0.391 | 0.911 | 2.33× |
| 10 | 1 | 0.041 | 0.086 | 2.09× |
| 10 | 5 | 0.180 | 0.402 | 2.23× |
| 10 | 10 | 0.382 | 0.902 | 2.36× |
| 100 | 1 | 0.043 | 0.086 | 2.01× |
| 100 | 5 | 0.174 | 0.400 | 2.30× |
| 100 | 10 | 0.382 | 0.875 | 2.29× |
| 1,000 | 1 | 0.046 | 0.106 | 2.29× |
| 1,000 | 5 | 0.204 | 0.469 | 2.29× |
| 1,000 | 10 | 0.390 | 0.934 | 2.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
