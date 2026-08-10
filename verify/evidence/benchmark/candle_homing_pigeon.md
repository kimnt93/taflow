# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.31M | 0.016 | 63.82M | 0.031 | 1.62× | 1.97× |
| 10,000 | 0.112 | 89.50M | 0.113 | 88.76M | 0.094 | 0.84× | 0.83× |
| 100,000 | 1.088 | 91.93M | 1.100 | 90.91M | 0.757 | 0.70× | 0.69× |
| 1,000,000 | 11.872 | 84.23M | 10.644 | 93.95M | 7.126 | 0.60× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.103 | 0.93× |
| 1 | 5 | 0.394 | 0.446 | 1.13× |
| 1 | 10 | 0.526 | 0.932 | 1.77× |
| 10 | 1 | 0.059 | 0.096 | 1.62× |
| 10 | 5 | 0.241 | 0.416 | 1.72× |
| 10 | 10 | 0.562 | 0.936 | 1.67× |
| 100 | 1 | 0.055 | 0.087 | 1.57× |
| 100 | 5 | 0.268 | 0.440 | 1.64× |
| 100 | 10 | 0.565 | 0.930 | 1.65× |
| 1,000 | 1 | 0.074 | 0.103 | 1.39× |
| 1,000 | 5 | 0.270 | 0.474 | 1.76× |
| 1,000 | 10 | 0.572 | 0.967 | 1.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
