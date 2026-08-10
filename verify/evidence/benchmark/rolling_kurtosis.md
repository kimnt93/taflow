# RollingKurtosis benchmark (`Kurtosis` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.33M | 0.019 | 51.31M | 0.166 | 8.18× | 8.51× |
| 10,000 | 0.163 | 61.18M | 0.161 | 62.01M | 0.522 | 3.19× | 3.24× |
| 100,000 | 1.554 | 64.34M | 1.560 | 64.09M | 4.066 | 2.62× | 2.61× |
| 1,000,000 | 16.264 | 61.49M | 16.695 | 59.90M | 40.070 | 2.46× | 2.40× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.245 | 2.26× |
| 1 | 5 | 0.400 | 1.214 | 3.03× |
| 1 | 10 | 0.473 | 2.267 | 4.79× |
| 10 | 1 | 0.050 | 0.221 | 4.38× |
| 10 | 5 | 0.224 | 1.226 | 5.48× |
| 10 | 10 | 0.479 | 2.238 | 4.67× |
| 100 | 1 | 0.057 | 0.221 | 3.90× |
| 100 | 5 | 0.245 | 1.244 | 5.07× |
| 100 | 10 | 0.487 | 2.372 | 4.87× |
| 1,000 | 1 | 0.071 | 0.255 | 3.60× |
| 1,000 | 5 | 0.247 | 1.432 | 5.79× |
| 1,000 | 10 | 0.504 | 2.601 | 5.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
