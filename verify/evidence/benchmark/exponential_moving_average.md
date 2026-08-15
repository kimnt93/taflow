# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 288.23M | 0.003 | 371.61M | 0.033 | 9.48× | 12.22× |
| 10,000 | 0.022 | 445.26M | 0.020 | 492.11M | 0.060 | 2.67× | 2.95× |
| 100,000 | 0.214 | 467.27M | 0.194 | 514.14M | 0.299 | 1.39× | 1.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.160 | 1.26× |
| 1 | 5 | 0.427 | 0.469 | 1.10× |
| 1 | 10 | 0.400 | 0.924 | 2.31× |
| 10 | 1 | 0.045 | 0.088 | 1.96× |
| 10 | 5 | 0.186 | 0.476 | 2.56× |
| 10 | 10 | 0.405 | 0.932 | 2.30× |
| 100 | 1 | 0.042 | 0.087 | 2.08× |
| 100 | 5 | 0.196 | 0.468 | 2.38× |
| 100 | 10 | 0.425 | 0.950 | 2.23× |
| 1,000 | 1 | 0.054 | 0.095 | 1.75× |
| 1,000 | 5 | 0.195 | 0.446 | 2.29× |
| 1,000 | 10 | 0.415 | 1.002 | 2.42× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
