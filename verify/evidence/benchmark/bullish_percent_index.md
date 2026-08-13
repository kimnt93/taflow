# BullishPercentIndex benchmark (`BullishPercentIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.47M | 0.031 | 32.29M | 11.758 | 323.04× | 379.70× |
| 10,000 | 0.261 | 38.35M | 0.246 | 40.67M | 108.991 | 418.01× | 443.32× |
| 100,000 | 2.235 | 44.74M | 2.194 | 45.58M | 1123.113 | 502.52× | 511.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | 0.267 | 1.61× |
| 1 | 5 | 0.481 | 1.037 | 2.16× |
| 1 | 10 | 0.606 | 2.080 | 3.43× |
| 10 | 1 | 0.071 | 0.319 | 4.53× |
| 10 | 5 | 0.294 | 1.822 | 6.20× |
| 10 | 10 | 0.564 | 3.164 | 5.61× |
| 100 | 1 | 0.070 | 1.369 | 19.44× |
| 100 | 5 | 0.300 | 7.018 | 23.38× |
| 100 | 10 | 0.644 | 13.811 | 21.44× |
| 1,000 | 1 | 0.098 | 11.745 | 119.27× |
| 1,000 | 5 | 0.514 | 63.818 | 124.20× |
| 1,000 | 10 | 0.792 | 126.252 | 159.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
