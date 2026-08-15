# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 283.46M | 0.003 | 387.30M | 0.034 | 9.64× | 13.18× |
| 10,000 | 0.020 | 492.53M | 0.017 | 601.22M | 0.042 | 2.05× | 2.51× |
| 100,000 | 0.177 | 566.46M | 0.157 | 637.91M | 0.126 | 0.72× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.116 | 1.02× |
| 1 | 5 | 0.235 | 0.519 | 2.21× |
| 1 | 10 | 0.406 | 0.967 | 2.38× |
| 10 | 1 | 0.040 | 0.087 | 2.18× |
| 10 | 5 | 0.184 | 0.448 | 2.43× |
| 10 | 10 | 0.401 | 0.982 | 2.45× |
| 100 | 1 | 0.043 | 0.095 | 2.21× |
| 100 | 5 | 0.214 | 0.445 | 2.08× |
| 100 | 10 | 0.404 | 1.014 | 2.51× |
| 1,000 | 1 | 0.061 | 0.100 | 1.65× |
| 1,000 | 5 | 0.216 | 0.531 | 2.45× |
| 1,000 | 10 | 0.440 | 0.979 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
