# RollingMedian benchmark (`MedianMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.289 | 3.46M | 0.276 | 3.62M | 0.314 | 1.09× | 1.14× |
| 10,000 | 2.855 | 3.50M | 2.723 | 3.67M | 1.748 | 0.61× | 0.64× |
| 100,000 | 28.099 | 3.56M | 29.365 | 3.41M | 17.636 | 0.63× | 0.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.269 | 2.22× |
| 1 | 5 | 0.389 | 1.069 | 2.75× |
| 1 | 10 | 0.611 | 2.362 | 3.87× |
| 10 | 1 | 0.068 | 0.206 | 3.03× |
| 10 | 5 | 0.312 | 1.022 | 3.27× |
| 10 | 10 | 0.621 | 2.219 | 3.57× |
| 100 | 1 | 0.110 | 0.230 | 2.10× |
| 100 | 5 | 0.296 | 1.313 | 4.44× |
| 100 | 10 | 0.648 | 2.426 | 3.75× |
| 1,000 | 1 | 0.379 | 0.397 | 1.05× |
| 1,000 | 5 | 0.688 | 2.273 | 3.30× |
| 1,000 | 10 | 1.121 | 4.173 | 3.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
