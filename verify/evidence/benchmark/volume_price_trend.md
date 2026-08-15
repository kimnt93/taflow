# VolumePriceTrend benchmark (`VolumePriceTrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 196.17M | 0.004 | 263.02M | 0.159 | 31.23× | 41.87× |
| 10,000 | 0.030 | 332.98M | 0.026 | 384.79M | 0.703 | 23.41× | 27.06× |
| 100,000 | 0.270 | 370.23M | 0.251 | 398.39M | 6.449 | 23.88× | 25.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.231 | 3.54× |
| 1 | 5 | 0.286 | 1.354 | 4.74× |
| 1 | 10 | 0.388 | 2.121 | 5.47× |
| 10 | 1 | 0.047 | 0.174 | 3.69× |
| 10 | 5 | 0.195 | 0.822 | 4.21× |
| 10 | 10 | 0.394 | 2.114 | 5.37× |
| 100 | 1 | 0.055 | 0.172 | 3.16× |
| 100 | 5 | 0.179 | 0.832 | 4.65× |
| 100 | 10 | 0.396 | 2.234 | 5.64× |
| 1,000 | 1 | 0.048 | 0.227 | 4.75× |
| 1,000 | 5 | 0.194 | 1.115 | 5.74× |
| 1,000 | 10 | 0.454 | 2.445 | 5.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
