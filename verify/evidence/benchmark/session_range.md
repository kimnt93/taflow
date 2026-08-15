# SessionRange benchmark (`SessionRange` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 46.46M | 0.017 | 60.32M | 0.703 | 32.64× | 42.37× |
| 10,000 | 0.158 | 63.21M | 0.145 | 68.89M | 5.744 | 36.31× | 39.57× |
| 100,000 | 1.717 | 58.26M | 1.602 | 62.41M | 60.228 | 35.09× | 37.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.294 | 3.69× |
| 1 | 5 | 0.339 | 1.146 | 3.38× |
| 1 | 10 | 0.403 | 2.443 | 6.07× |
| 10 | 1 | 0.046 | 0.236 | 5.16× |
| 10 | 5 | 0.208 | 1.430 | 6.89× |
| 10 | 10 | 0.470 | 2.500 | 5.32× |
| 100 | 1 | 0.048 | 0.284 | 5.91× |
| 100 | 5 | 0.226 | 1.667 | 7.39× |
| 100 | 10 | 0.424 | 3.023 | 7.14× |
| 1,000 | 1 | 0.067 | 0.940 | 14.06× |
| 1,000 | 5 | 0.204 | 4.288 | 21.02× |
| 1,000 | 10 | 0.449 | 15.427 | 34.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
