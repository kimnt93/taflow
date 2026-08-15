# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 205.68M | 0.004 | 247.32M | 0.033 | 6.77× | 8.15× |
| 10,000 | 0.035 | 289.76M | 0.032 | 312.70M | 0.053 | 1.53× | 1.65× |
| 100,000 | 0.348 | 287.73M | 0.306 | 327.05M | 0.216 | 0.62× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.197 | 0.117 | 0.59× |
| 1 | 5 | 0.248 | 0.516 | 2.08× |
| 1 | 10 | 0.410 | 0.952 | 2.32× |
| 10 | 1 | 0.043 | 0.088 | 2.04× |
| 10 | 5 | 0.185 | 0.498 | 2.69× |
| 10 | 10 | 0.399 | 1.016 | 2.55× |
| 100 | 1 | 0.043 | 0.091 | 2.10× |
| 100 | 5 | 0.183 | 0.418 | 2.28× |
| 100 | 10 | 0.408 | 0.989 | 2.42× |
| 1,000 | 1 | 0.058 | 0.098 | 1.69× |
| 1,000 | 5 | 0.227 | 0.468 | 2.06× |
| 1,000 | 10 | 0.411 | 0.951 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
