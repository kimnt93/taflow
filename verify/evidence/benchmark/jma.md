# JurikMovingAverage benchmark (`jma` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.087 | 11.56M | 0.090 | 11.11M | 20.205 | 233.50× | 224.48× |
| 10,000 | 0.846 | 11.82M | 0.863 | 11.59M | 209.160 | 247.31× | 242.39× |
| 100,000 | 9.333 | 10.71M | 8.966 | 11.15M | 1991.366 | 213.36× | 222.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.192 | 1.46× |
| 1 | 5 | 0.230 | 0.887 | 3.85× |
| 1 | 10 | 0.420 | 1.711 | 4.07× |
| 10 | 1 | 0.047 | 0.632 | 13.46× |
| 10 | 5 | 0.222 | 2.724 | 12.29× |
| 10 | 10 | 0.408 | 5.009 | 12.27× |
| 100 | 1 | 0.051 | 2.440 | 47.53× |
| 100 | 5 | 0.224 | 12.708 | 56.73× |
| 100 | 10 | 0.460 | 24.947 | 54.23× |
| 1,000 | 1 | 0.134 | 21.028 | 156.45× |
| 1,000 | 5 | 0.418 | 108.146 | 258.51× |
| 1,000 | 10 | 0.656 | 213.593 | 325.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
