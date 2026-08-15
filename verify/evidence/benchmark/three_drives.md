# ThreeDrives benchmark (`ThreeDrives` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.26M | 0.010 | 103.85M | 0.290 | 21.82× | 30.11× |
| 10,000 | 0.117 | 85.75M | 0.112 | 89.01M | 1.550 | 13.29× | 13.79× |
| 100,000 | 1.043 | 95.91M | 0.969 | 103.19M | 13.833 | 13.27× | 14.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.197 | 2.66× |
| 1 | 5 | 0.236 | 0.780 | 3.31× |
| 1 | 10 | 0.484 | 1.830 | 3.78× |
| 10 | 1 | 0.048 | 0.164 | 3.43× |
| 10 | 5 | 0.207 | 1.087 | 5.24× |
| 10 | 10 | 0.423 | 1.756 | 4.15× |
| 100 | 1 | 0.064 | 0.192 | 3.01× |
| 100 | 5 | 0.233 | 1.269 | 5.45× |
| 100 | 10 | 0.405 | 1.966 | 4.85× |
| 1,000 | 1 | 0.060 | 0.332 | 5.54× |
| 1,000 | 5 | 0.228 | 1.773 | 7.79× |
| 1,000 | 10 | 0.504 | 3.355 | 6.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
