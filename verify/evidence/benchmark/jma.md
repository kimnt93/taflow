# JurikMovingAverage benchmark (`jma` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.098 | 10.25M | 0.099 | 10.13M | 22.875 | 234.49× | 231.65× |
| 10,000 | 0.955 | 10.47M | 0.922 | 10.85M | 213.331 | 223.40× | 231.43× |
| 100,000 | 9.226 | 10.84M | 8.843 | 11.31M | 2013.505 | 218.24× | 227.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.250 | 3.13× |
| 1 | 5 | 0.344 | 1.005 | 2.92× |
| 1 | 10 | 0.502 | 1.729 | 3.45× |
| 10 | 1 | 0.052 | 0.518 | 9.93× |
| 10 | 5 | 0.271 | 2.677 | 9.89× |
| 10 | 10 | 0.581 | 5.178 | 8.91× |
| 100 | 1 | 0.061 | 2.409 | 39.53× |
| 100 | 5 | 0.247 | 11.923 | 48.24× |
| 100 | 10 | 0.562 | 24.076 | 42.85× |
| 1,000 | 1 | 0.176 | 19.948 | 113.25× |
| 1,000 | 5 | 0.385 | 103.764 | 269.18× |
| 1,000 | 10 | 0.656 | 218.156 | 332.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
