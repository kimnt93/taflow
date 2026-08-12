# FractalDimension benchmark (`two-chunk rescaled-range dimension` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.216 | 4.63M | 0.228 | 4.38M | 0.839 | 3.89× | 3.67× |
| 10,000 | 2.259 | 4.43M | 2.349 | 4.26M | 5.772 | 2.56× | 2.46× |
| 100,000 | 23.833 | 4.20M | 24.164 | 4.14M | 68.597 | 2.88× | 2.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.108 | 1.13× |
| 1 | 5 | 0.300 | 0.484 | 1.61× |
| 1 | 10 | 0.468 | 1.127 | 2.41× |
| 10 | 1 | 0.052 | 0.083 | 1.60× |
| 10 | 5 | 0.225 | 0.435 | 1.93× |
| 10 | 10 | 0.490 | 0.869 | 1.77× |
| 100 | 1 | 0.067 | 0.376 | 5.59× |
| 100 | 5 | 0.249 | 2.202 | 8.85× |
| 100 | 10 | 0.525 | 4.376 | 8.34× |
| 1,000 | 1 | 0.284 | 0.899 | 3.17× |
| 1,000 | 5 | 0.443 | 2.907 | 6.56× |
| 1,000 | 10 | 0.708 | 6.579 | 9.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
