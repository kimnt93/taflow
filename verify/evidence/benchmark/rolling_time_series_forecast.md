# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.53M | 0.014 | 74.02M | 0.046 | 3.03× | 3.37× |
| 10,000 | 0.125 | 80.31M | 0.126 | 79.66M | 0.158 | 1.27× | 1.26× |
| 100,000 | 1.261 | 79.28M | 1.220 | 81.96M | 1.293 | 1.02× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.174 | 1.92× |
| 1 | 5 | 0.233 | 0.517 | 2.22× |
| 1 | 10 | 0.398 | 0.946 | 2.38× |
| 10 | 1 | 0.041 | 0.088 | 2.14× |
| 10 | 5 | 0.178 | 0.441 | 2.48× |
| 10 | 10 | 0.385 | 0.935 | 2.43× |
| 100 | 1 | 0.044 | 0.091 | 2.10× |
| 100 | 5 | 0.185 | 0.463 | 2.51× |
| 100 | 10 | 0.396 | 0.973 | 2.46× |
| 1,000 | 1 | 0.063 | 0.115 | 1.83× |
| 1,000 | 5 | 0.207 | 0.529 | 2.56× |
| 1,000 | 10 | 0.423 | 1.074 | 2.54× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
