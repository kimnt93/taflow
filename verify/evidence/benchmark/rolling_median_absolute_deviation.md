# RollingMedianAbsoluteDeviation benchmark (`MedianAbsoluteDeviation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.324 | 3.08M | 0.326 | 3.07M | 0.509 | 1.57× | 1.56× |
| 10,000 | 3.215 | 3.11M | 3.278 | 3.05M | 3.771 | 1.17× | 1.15× |
| 100,000 | 31.983 | 3.13M | 31.971 | 3.13M | 35.185 | 1.10× | 1.10× |
| 1,000,000 | 327.035 | 3.06M | 321.383 | 3.11M | 354.037 | 1.08× | 1.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.162 | 0.306 | 1.89× |
| 1 | 5 | 0.327 | 1.396 | 4.27× |
| 1 | 10 | 0.515 | 2.994 | 5.82× |
| 10 | 1 | 0.057 | 0.230 | 4.04× |
| 10 | 5 | 0.287 | 1.202 | 4.19× |
| 10 | 10 | 0.515 | 2.521 | 4.90× |
| 100 | 1 | 0.097 | 0.246 | 2.53× |
| 100 | 5 | 0.256 | 1.507 | 5.89× |
| 100 | 10 | 0.593 | 2.827 | 4.76× |
| 1,000 | 1 | 0.392 | 0.602 | 1.53× |
| 1,000 | 5 | 0.758 | 3.230 | 4.26× |
| 1,000 | 10 | 1.191 | 6.033 | 5.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
