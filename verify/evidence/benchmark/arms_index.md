# ArmsIndex benchmark (`Trin` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 134.81M | 0.005 | 202.83M | 8.495 | 1145.24× | 1723.11× |
| 10,000 | 0.041 | 243.48M | 0.037 | 270.10M | 85.219 | 2074.91× | 2301.74× |
| 100,000 | 0.373 | 268.37M | 0.339 | 294.69M | 835.010 | 2240.93× | 2460.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.185 | 0.367 | 1.98× |
| 1 | 5 | 0.338 | 1.288 | 3.81× |
| 1 | 10 | 0.401 | 2.566 | 6.41× |
| 10 | 1 | 0.044 | 0.303 | 6.93× |
| 10 | 5 | 0.185 | 1.510 | 8.14× |
| 10 | 10 | 0.392 | 3.386 | 8.63× |
| 100 | 1 | 0.045 | 1.072 | 23.83× |
| 100 | 5 | 0.212 | 5.595 | 26.34× |
| 100 | 10 | 0.434 | 11.545 | 26.61× |
| 1,000 | 1 | 0.055 | 8.848 | 160.77× |
| 1,000 | 5 | 0.269 | 46.903 | 174.67× |
| 1,000 | 10 | 0.647 | 92.459 | 142.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
