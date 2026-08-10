# MassIndex benchmark (`MassIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.82M | 0.013 | 78.22M | 0.264 | 19.20× | 20.63× |
| 10,000 | 0.094 | 106.35M | 0.088 | 113.60M | 0.814 | 8.66× | 9.25× |
| 100,000 | 0.947 | 105.62M | 0.873 | 114.61M | 11.236 | 11.87× | 12.88× |
| 1,000,000 | 9.112 | 109.75M | 8.421 | 118.76M | 65.052 | 7.14× | 7.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.309 | 2.97× |
| 1 | 5 | 0.303 | 1.502 | 4.95× |
| 1 | 10 | 0.523 | 3.020 | 5.78× |
| 10 | 1 | 0.060 | 0.255 | 4.23× |
| 10 | 5 | 0.244 | 1.513 | 6.20× |
| 10 | 10 | 0.529 | 2.840 | 5.37× |
| 100 | 1 | 0.061 | 0.266 | 4.37× |
| 100 | 5 | 0.254 | 1.765 | 6.95× |
| 100 | 10 | 0.545 | 3.058 | 5.61× |
| 1,000 | 1 | 0.072 | 0.323 | 4.47× |
| 1,000 | 5 | 0.300 | 1.887 | 6.30× |
| 1,000 | 10 | 0.546 | 3.362 | 6.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
