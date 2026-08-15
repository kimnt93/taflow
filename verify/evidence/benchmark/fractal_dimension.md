# FractalDimension benchmark (`two-chunk rescaled-range dimension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.185 | 5.39M | 0.193 | 5.17M | 0.781 | 4.21× | 4.04× |
| 10,000 | 1.902 | 5.26M | 1.873 | 5.34M | 5.462 | 2.87× | 2.92× |
| 100,000 | 18.843 | 5.31M | 18.887 | 5.29M | 62.806 | 3.33× | 3.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.124 | 1.54× |
| 1 | 5 | 0.273 | 0.413 | 1.51× |
| 1 | 10 | 0.391 | 1.087 | 2.78× |
| 10 | 1 | 0.049 | 0.082 | 1.66× |
| 10 | 5 | 0.179 | 0.418 | 2.34× |
| 10 | 10 | 0.415 | 0.854 | 2.06× |
| 100 | 1 | 0.056 | 0.386 | 6.86× |
| 100 | 5 | 0.194 | 1.987 | 10.23× |
| 100 | 10 | 0.419 | 4.282 | 10.23× |
| 1,000 | 1 | 0.250 | 0.880 | 3.52× |
| 1,000 | 5 | 0.476 | 3.530 | 7.42× |
| 1,000 | 10 | 0.652 | 6.958 | 10.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
