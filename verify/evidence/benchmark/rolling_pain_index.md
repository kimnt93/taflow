# RollingPainIndex benchmark (`PainIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.53M | 0.048 | 20.90M | 0.168 | 3.61× | 3.51× |
| 10,000 | 0.448 | 22.33M | 0.449 | 22.25M | 0.674 | 1.51× | 1.50× |
| 100,000 | 4.446 | 22.49M | 4.470 | 22.37M | 6.146 | 1.38× | 1.37× |
| 1,000,000 | 46.922 | 21.31M | 46.480 | 21.51M | 58.366 | 1.24× | 1.26× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.298 | 3.29× |
| 1 | 5 | 0.313 | 0.998 | 3.19× |
| 1 | 10 | 0.479 | 2.081 | 4.35× |
| 10 | 1 | 0.053 | 0.189 | 3.55× |
| 10 | 5 | 0.218 | 0.951 | 4.36× |
| 10 | 10 | 0.465 | 2.137 | 4.59× |
| 100 | 1 | 0.055 | 0.192 | 3.51× |
| 100 | 5 | 0.228 | 0.970 | 4.26× |
| 100 | 10 | 0.487 | 2.148 | 4.41× |
| 1,000 | 1 | 0.098 | 0.244 | 2.50× |
| 1,000 | 5 | 0.235 | 1.238 | 5.27× |
| 1,000 | 10 | 0.537 | 2.747 | 5.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
