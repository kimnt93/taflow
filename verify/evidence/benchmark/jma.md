# JurikMovingAverage benchmark (`jma` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.174 | 5.73M | 0.168 | 5.95M | 19.304 | 110.67× | 114.79× |
| 10,000 | 1.611 | 6.21M | 1.657 | 6.04M | 205.914 | 127.83× | 124.29× |
| 100,000 | 17.099 | 5.85M | 16.523 | 6.05M | 1927.411 | 112.72× | 116.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.221 | 0.239 | 1.08× |
| 1 | 5 | 0.574 | 0.840 | 1.46× |
| 1 | 10 | 0.618 | 1.604 | 2.60× |
| 10 | 1 | 0.073 | 0.514 | 7.08× |
| 10 | 5 | 0.317 | 2.403 | 7.58× |
| 10 | 10 | 0.637 | 4.751 | 7.46× |
| 100 | 1 | 0.087 | 2.307 | 26.37× |
| 100 | 5 | 0.310 | 11.382 | 36.75× |
| 100 | 10 | 0.796 | 23.769 | 29.85× |
| 1,000 | 1 | 0.298 | 20.050 | 67.37× |
| 1,000 | 5 | 0.669 | 125.616 | 187.83× |
| 1,000 | 10 | 1.111 | 296.618 | 267.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
