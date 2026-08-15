# LogReturn benchmark (`LogReturn` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 114.56M | 0.008 | 122.18M | 0.186 | 21.35× | 22.77× |
| 10,000 | 0.078 | 128.16M | 0.076 | 132.11M | 0.574 | 7.35× | 7.58× |
| 100,000 | 0.895 | 111.78M | 1.240 | 80.64M | 7.807 | 8.73× | 6.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.303 | 4.53× |
| 1 | 5 | 0.287 | 1.327 | 4.62× |
| 1 | 10 | 0.440 | 2.995 | 6.81× |
| 10 | 1 | 0.067 | 0.282 | 4.21× |
| 10 | 5 | 0.271 | 1.764 | 6.51× |
| 10 | 10 | 0.539 | 3.087 | 5.73× |
| 100 | 1 | 0.051 | 0.240 | 4.69× |
| 100 | 5 | 0.265 | 1.689 | 6.37× |
| 100 | 10 | 0.503 | 9.887 | 19.66× |
| 1,000 | 1 | 0.060 | 0.275 | 4.55× |
| 1,000 | 5 | 0.243 | 1.490 | 6.13× |
| 1,000 | 10 | 0.483 | 5.482 | 11.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
