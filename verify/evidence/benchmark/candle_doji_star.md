# CandleDojiStar benchmark (`CDLDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.33M | 0.017 | 60.24M | 0.042 | 2.11× | 2.53× |
| 10,000 | 0.166 | 60.16M | 0.157 | 63.79M | 0.134 | 0.81× | 0.86× |
| 100,000 | 1.605 | 62.30M | 1.709 | 58.53M | 1.076 | 0.67× | 0.63× |
| 1,000,000 | 17.078 | 58.56M | 16.959 | 58.97M | 10.749 | 0.63× | 0.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.121 | 1.14× |
| 1 | 5 | 0.339 | 0.506 | 1.49× |
| 1 | 10 | 0.523 | 0.916 | 1.75× |
| 10 | 1 | 0.059 | 0.090 | 1.53× |
| 10 | 5 | 0.253 | 0.446 | 1.76× |
| 10 | 10 | 0.593 | 0.922 | 1.55× |
| 100 | 1 | 0.067 | 0.088 | 1.32× |
| 100 | 5 | 0.258 | 0.435 | 1.68× |
| 100 | 10 | 0.573 | 0.975 | 1.70× |
| 1,000 | 1 | 0.071 | 0.098 | 1.37× |
| 1,000 | 5 | 0.280 | 0.501 | 1.79× |
| 1,000 | 10 | 0.553 | 0.999 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
