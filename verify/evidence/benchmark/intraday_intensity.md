# IntradayIntensity benchmark (`IntradayIntensity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.83M | 0.009 | 112.14M | 0.237 | 20.83× | 26.59× |
| 10,000 | 0.040 | 251.59M | 0.036 | 275.19M | 1.373 | 34.55× | 37.79× |
| 100,000 | 0.310 | 322.70M | 0.282 | 354.39M | 12.822 | 41.38× | 45.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.248 | 2.17× |
| 1 | 5 | 0.306 | 0.837 | 2.74× |
| 1 | 10 | 0.560 | 1.780 | 3.18× |
| 10 | 1 | 0.062 | 0.169 | 2.74× |
| 10 | 5 | 0.254 | 1.093 | 4.31× |
| 10 | 10 | 0.582 | 1.790 | 3.07× |
| 100 | 1 | 0.068 | 0.176 | 2.57× |
| 100 | 5 | 0.248 | 1.265 | 5.10× |
| 100 | 10 | 0.605 | 1.915 | 3.16× |
| 1,000 | 1 | 0.065 | 0.301 | 4.63× |
| 1,000 | 5 | 0.267 | 1.850 | 6.93× |
| 1,000 | 10 | 0.585 | 3.096 | 5.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
