# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.91M | 0.004 | 244.09M | 0.032 | 6.14× | 7.93× |
| 10,000 | 0.037 | 273.76M | 0.033 | 298.55M | 0.050 | 1.36× | 1.48× |
| 100,000 | 0.352 | 283.95M | 0.323 | 309.56M | 0.214 | 0.61× | 0.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.117 | 1.11× |
| 1 | 5 | 0.202 | 0.439 | 2.17× |
| 1 | 10 | 0.396 | 0.926 | 2.34× |
| 10 | 1 | 0.040 | 0.087 | 2.18× |
| 10 | 5 | 0.191 | 0.462 | 2.42× |
| 10 | 10 | 0.436 | 0.973 | 2.23× |
| 100 | 1 | 0.041 | 0.090 | 2.20× |
| 100 | 5 | 0.180 | 0.421 | 2.34× |
| 100 | 10 | 0.417 | 0.966 | 2.32× |
| 1,000 | 1 | 0.049 | 0.095 | 1.96× |
| 1,000 | 5 | 0.202 | 0.452 | 2.24× |
| 1,000 | 10 | 0.434 | 0.994 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
