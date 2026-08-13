# RollingCalmar benchmark (`rolling calmar on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.195 | 5.13M | 0.188 | 5.32M | 0.228 | 1.17× | 1.22× |
| 10,000 | 1.804 | 5.54M | 1.804 | 5.54M | 1.400 | 0.78× | 0.78× |
| 100,000 | 18.083 | 5.53M | 18.026 | 5.55M | 16.064 | 0.89× | 0.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.154 | 1.55× |
| 1 | 5 | 0.431 | 0.440 | 1.02× |
| 1 | 10 | 0.594 | 0.983 | 1.65× |
| 10 | 1 | 0.064 | 0.088 | 1.37× |
| 10 | 5 | 0.299 | 0.410 | 1.37× |
| 10 | 10 | 0.638 | 0.858 | 1.35× |
| 100 | 1 | 0.083 | 0.198 | 2.38× |
| 100 | 5 | 0.317 | 1.863 | 5.89× |
| 100 | 10 | 0.772 | 2.092 | 2.71× |
| 1,000 | 1 | 0.279 | 0.326 | 1.17× |
| 1,000 | 5 | 0.468 | 1.219 | 2.61× |
| 1,000 | 10 | 0.844 | 2.427 | 2.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
