# RollingWinsorize benchmark (`rolling winsorize` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.40M | 0.045 | 22.38M | 0.593 | 12.70× | 13.28× |
| 10,000 | 0.496 | 20.17M | 0.501 | 19.95M | 3.322 | 6.70× | 6.63× |
| 100,000 | 5.126 | 19.51M | 5.022 | 19.91M | 33.264 | 6.49× | 6.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.360 | 4.76× |
| 1 | 5 | 0.199 | 1.649 | 8.28× |
| 1 | 10 | 0.389 | 3.365 | 8.65× |
| 10 | 1 | 0.048 | 0.296 | 6.20× |
| 10 | 5 | 0.192 | 1.661 | 8.65× |
| 10 | 10 | 0.397 | 3.366 | 8.48× |
| 100 | 1 | 0.054 | 0.380 | 7.03× |
| 100 | 5 | 0.211 | 1.997 | 9.45× |
| 100 | 10 | 0.445 | 4.084 | 9.17× |
| 1,000 | 1 | 0.098 | 0.661 | 6.78× |
| 1,000 | 5 | 0.248 | 2.328 | 9.39× |
| 1,000 | 10 | 0.456 | 4.895 | 10.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
