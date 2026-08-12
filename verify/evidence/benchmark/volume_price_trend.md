# VolumePriceTrend benchmark (`VolumePriceTrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.36M | 0.006 | 157.30M | 0.176 | 23.87× | 27.74× |
| 10,000 | 0.037 | 268.48M | 0.031 | 324.47M | 0.744 | 19.97× | 24.14× |
| 100,000 | 0.315 | 317.15M | 0.285 | 351.28M | 7.061 | 22.39× | 24.80× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.264 | 2.25× |
| 1 | 5 | 0.228 | 1.354 | 5.94× |
| 1 | 10 | 0.493 | 2.265 | 4.60× |
| 10 | 1 | 0.064 | 0.177 | 2.77× |
| 10 | 5 | 0.227 | 0.881 | 3.88× |
| 10 | 10 | 0.543 | 2.262 | 4.17× |
| 100 | 1 | 0.059 | 0.188 | 3.20× |
| 100 | 5 | 0.231 | 0.891 | 3.86× |
| 100 | 10 | 0.611 | 2.224 | 3.64× |
| 1,000 | 1 | 0.053 | 0.231 | 4.35× |
| 1,000 | 5 | 0.266 | 1.309 | 4.93× |
| 1,000 | 10 | 0.576 | 2.549 | 4.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
