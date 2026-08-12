# HighestSince benchmark (`highest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.94M | 0.007 | 143.89M | 0.319 | 37.28× | 45.87× |
| 10,000 | 0.042 | 236.19M | 0.041 | 245.61M | 4.329 | 102.24× | 106.32× |
| 100,000 | 0.375 | 266.32M | 0.338 | 296.14M | 28.826 | 76.77× | 85.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.088 | 0.79× |
| 1 | 5 | 0.253 | 0.327 | 1.29× |
| 1 | 10 | 0.511 | 0.670 | 1.31× |
| 10 | 1 | 0.049 | 0.070 | 1.43× |
| 10 | 5 | 0.245 | 0.330 | 1.35× |
| 10 | 10 | 0.455 | 0.705 | 1.55× |
| 100 | 1 | 0.056 | 0.114 | 2.04× |
| 100 | 5 | 0.234 | 0.466 | 1.99× |
| 100 | 10 | 0.456 | 0.921 | 2.02× |
| 1,000 | 1 | 0.058 | 0.349 | 6.06× |
| 1,000 | 5 | 0.242 | 1.821 | 7.54× |
| 1,000 | 10 | 0.520 | 3.608 | 6.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
