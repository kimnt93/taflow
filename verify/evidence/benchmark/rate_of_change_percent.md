# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 445.72M | 0.001 | 704.47M | 0.032 | 14.12× | 22.31× |
| 10,000 | 0.008 | 1.20G | 0.006 | 1.70G | 0.042 | 5.04× | 7.14× |
| 100,000 | 0.075 | 1.33G | 0.049 | 2.04G | 0.129 | 1.72× | 2.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.152 | 2.24× |
| 1 | 5 | 0.301 | 0.513 | 1.71× |
| 1 | 10 | 0.443 | 1.012 | 2.28× |
| 10 | 1 | 0.043 | 0.087 | 2.02× |
| 10 | 5 | 0.179 | 0.431 | 2.41× |
| 10 | 10 | 0.393 | 1.050 | 2.67× |
| 100 | 1 | 0.056 | 0.088 | 1.57× |
| 100 | 5 | 0.197 | 0.437 | 2.22× |
| 100 | 10 | 0.415 | 0.934 | 2.25× |
| 1,000 | 1 | 0.051 | 0.103 | 2.03× |
| 1,000 | 5 | 0.196 | 0.439 | 2.25× |
| 1,000 | 10 | 0.378 | 0.951 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
