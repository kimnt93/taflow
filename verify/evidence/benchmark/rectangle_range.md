# RectangleRange benchmark (`RectangleRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.93M | 0.014 | 68.98M | 0.232 | 11.84× | 16.03× |
| 10,000 | 0.105 | 95.49M | 0.101 | 99.39M | 2.344 | 22.38× | 23.30× |
| 100,000 | 1.055 | 94.75M | 0.998 | 100.24M | 15.047 | 14.26× | 15.08× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.286 | 2.55× |
| 1 | 5 | 0.320 | 0.948 | 2.96× |
| 1 | 10 | 0.592 | 1.875 | 3.17× |
| 10 | 1 | 0.057 | 0.180 | 3.16× |
| 10 | 5 | 0.253 | 1.091 | 4.30× |
| 10 | 10 | 0.530 | 1.810 | 3.41× |
| 100 | 1 | 0.063 | 0.194 | 3.06× |
| 100 | 5 | 0.253 | 1.179 | 4.67× |
| 100 | 10 | 0.550 | 1.893 | 3.44× |
| 1,000 | 1 | 0.066 | 0.302 | 4.56× |
| 1,000 | 5 | 0.255 | 1.790 | 7.03× |
| 1,000 | 10 | 0.600 | 3.060 | 5.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
