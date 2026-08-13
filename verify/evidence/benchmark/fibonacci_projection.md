# FibonacciProjection benchmark (`FibProjection` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.098 | 10.18M | 0.086 | 11.59M | 0.542 | 5.51× | 6.28× |
| 10,000 | 0.787 | 12.71M | 0.775 | 12.91M | 4.286 | 5.45× | 5.53× |
| 100,000 | 7.960 | 12.56M | 7.546 | 13.25M | 45.705 | 5.74× | 6.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.208 | 2.21× |
| 1 | 5 | 0.379 | 0.902 | 2.38× |
| 1 | 10 | 0.622 | 1.884 | 3.03× |
| 10 | 1 | 0.075 | 0.170 | 2.26× |
| 10 | 5 | 0.301 | 0.840 | 2.79× |
| 10 | 10 | 0.619 | 1.897 | 3.06× |
| 100 | 1 | 0.083 | 0.216 | 2.62× |
| 100 | 5 | 0.296 | 1.055 | 3.56× |
| 100 | 10 | 0.632 | 2.336 | 3.69× |
| 1,000 | 1 | 0.164 | 0.827 | 5.04× |
| 1,000 | 5 | 0.312 | 3.364 | 10.78× |
| 1,000 | 10 | 0.682 | 6.738 | 9.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
