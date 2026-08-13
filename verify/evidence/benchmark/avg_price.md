# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.25M | 0.038 | 26.26M | 0.032 | 0.70× | 0.83× |
| 10,000 | 0.269 | 37.17M | 0.256 | 39.01M | 0.036 | 0.13× | 0.14× |
| 100,000 | 2.537 | 39.42M | 2.425 | 41.24M | 0.104 | 0.04× | 0.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.116 | 1.01× |
| 1 | 5 | 0.455 | 0.478 | 1.05× |
| 1 | 10 | 0.652 | 0.897 | 1.38× |
| 10 | 1 | 0.068 | 0.085 | 1.26× |
| 10 | 5 | 0.306 | 0.418 | 1.37× |
| 10 | 10 | 0.628 | 0.904 | 1.44× |
| 100 | 1 | 0.070 | 0.087 | 1.24× |
| 100 | 5 | 0.323 | 0.416 | 1.29× |
| 100 | 10 | 0.618 | 0.891 | 1.44× |
| 1,000 | 1 | 0.093 | 0.095 | 1.03× |
| 1,000 | 5 | 0.329 | 0.429 | 1.30× |
| 1,000 | 10 | 0.666 | 0.915 | 1.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
