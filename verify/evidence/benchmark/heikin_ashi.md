# HeikinAshi benchmark (`HeikinAshi` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.078 | 12.79M | 0.070 | 14.38M | 0.553 | 7.07× | 7.95× |
| 10,000 | 0.588 | 17.00M | 0.579 | 17.26M | 4.551 | 7.74× | 7.86× |
| 100,000 | 5.796 | 17.25M | 5.609 | 17.83M | 51.507 | 8.89× | 9.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.228 | 1.77× |
| 1 | 5 | 0.404 | 0.892 | 2.21× |
| 1 | 10 | 0.631 | 2.037 | 3.23× |
| 10 | 1 | 0.080 | 0.177 | 2.22× |
| 10 | 5 | 0.306 | 0.882 | 2.89× |
| 10 | 10 | 0.638 | 2.047 | 3.21× |
| 100 | 1 | 0.087 | 0.223 | 2.57× |
| 100 | 5 | 0.326 | 1.120 | 3.44× |
| 100 | 10 | 0.632 | 2.481 | 3.92× |
| 1,000 | 1 | 0.140 | 0.936 | 6.70× |
| 1,000 | 5 | 0.298 | 3.639 | 12.23× |
| 1,000 | 10 | 0.663 | 19.314 | 29.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
