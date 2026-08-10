# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.74M | 0.010 | 103.60M | 0.036 | 2.45× | 3.69× |
| 10,000 | 0.046 | 216.53M | 0.041 | 242.52M | 0.054 | 1.17× | 1.31× |
| 100,000 | 0.366 | 273.03M | 0.392 | 255.16M | 0.262 | 0.72× | 0.67× |
| 1,000,000 | 4.234 | 236.17M | 4.132 | 242.01M | 3.064 | 0.72× | 0.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.161 | 1.74× |
| 1 | 5 | 0.358 | 0.695 | 1.94× |
| 1 | 10 | 0.536 | 1.037 | 1.94× |
| 10 | 1 | 0.069 | 0.095 | 1.37× |
| 10 | 5 | 0.367 | 0.538 | 1.46× |
| 10 | 10 | 0.573 | 0.974 | 1.70× |
| 100 | 1 | 0.059 | 0.112 | 1.91× |
| 100 | 5 | 0.329 | 0.559 | 1.70× |
| 100 | 10 | 0.612 | 0.956 | 1.56× |
| 1,000 | 1 | 0.058 | 0.092 | 1.59× |
| 1,000 | 5 | 0.283 | 1.087 | 3.83× |
| 1,000 | 10 | 0.679 | 0.967 | 1.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
