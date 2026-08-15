# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.10M | 0.007 | 153.45M | 0.033 | 4.42× | 5.02× |
| 10,000 | 0.070 | 142.75M | 0.063 | 159.77M | 0.091 | 1.30× | 1.45× |
| 100,000 | 0.732 | 136.69M | 0.632 | 158.28M | 0.696 | 0.95× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.146 | 1.57× |
| 1 | 5 | 0.262 | 0.456 | 1.74× |
| 1 | 10 | 0.411 | 0.892 | 2.17× |
| 10 | 1 | 0.040 | 0.090 | 2.24× |
| 10 | 5 | 0.178 | 0.394 | 2.21× |
| 10 | 10 | 0.392 | 0.944 | 2.41× |
| 100 | 1 | 0.046 | 0.086 | 1.87× |
| 100 | 5 | 0.180 | 0.430 | 2.39× |
| 100 | 10 | 0.406 | 0.845 | 2.08× |
| 1,000 | 1 | 0.057 | 0.090 | 1.58× |
| 1,000 | 5 | 0.201 | 0.476 | 2.36× |
| 1,000 | 10 | 0.408 | 0.947 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
