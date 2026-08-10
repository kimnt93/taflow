# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.36M | 0.011 | 87.99M | 0.035 | 2.42× | 3.07× |
| 10,000 | 0.073 | 136.52M | 0.070 | 141.97M | 0.097 | 1.32× | 1.37× |
| 100,000 | 0.768 | 130.26M | 0.752 | 132.96M | 0.708 | 0.92× | 0.94× |
| 1,000,000 | 8.606 | 116.20M | 7.558 | 132.31M | 6.649 | 0.77× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.164 | 0.173 | 1.06× |
| 1 | 5 | 0.301 | 0.498 | 1.65× |
| 1 | 10 | 0.638 | 0.923 | 1.45× |
| 10 | 1 | 0.053 | 0.087 | 1.64× |
| 10 | 5 | 0.252 | 0.417 | 1.66× |
| 10 | 10 | 1.003 | 0.956 | 0.95× |
| 100 | 1 | 0.066 | 0.091 | 1.38× |
| 100 | 5 | 0.240 | 0.436 | 1.82× |
| 100 | 10 | 0.560 | 0.984 | 1.76× |
| 1,000 | 1 | 0.068 | 0.103 | 1.51× |
| 1,000 | 5 | 0.281 | 0.501 | 1.79× |
| 1,000 | 10 | 0.608 | 1.031 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
