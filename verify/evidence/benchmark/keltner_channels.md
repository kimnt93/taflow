# KeltnerChannels benchmark (`Keltner` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.70M | 0.019 | 53.63M | 0.644 | 29.45× | 34.56× |
| 10,000 | 0.137 | 72.85M | 0.133 | 75.06M | 4.251 | 30.97× | 31.91× |
| 100,000 | 1.300 | 76.89M | 1.200 | 83.36M | 48.036 | 36.94× | 40.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.372 | 4.55× |
| 1 | 5 | 0.375 | 1.652 | 4.40× |
| 1 | 10 | 0.525 | 3.488 | 6.65× |
| 10 | 1 | 0.056 | 0.298 | 5.36× |
| 10 | 5 | 0.280 | 1.737 | 6.20× |
| 10 | 10 | 0.563 | 3.601 | 6.40× |
| 100 | 1 | 0.065 | 0.343 | 5.24× |
| 100 | 5 | 0.253 | 1.894 | 7.49× |
| 100 | 10 | 0.598 | 3.889 | 6.50× |
| 1,000 | 1 | 0.073 | 0.850 | 11.72× |
| 1,000 | 5 | 0.288 | 4.100 | 14.23× |
| 1,000 | 10 | 0.608 | 8.342 | 13.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
