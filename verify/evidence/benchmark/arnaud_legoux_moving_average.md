# ArnaudLegouxMovingAverage benchmark (`ALMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.334 | 2.99M | 0.339 | 2.95M | 0.222 | 0.67× | 0.66× |
| 10,000 | 3.305 | 3.03M | 3.290 | 3.04M | 0.570 | 0.17× | 0.17× |
| 100,000 | 33.318 | 3.00M | 33.634 | 2.97M | 4.141 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.296 | 0.388 | 1.31× |
| 1 | 5 | 0.360 | 1.475 | 4.10× |
| 1 | 10 | 0.659 | 2.957 | 4.49× |
| 10 | 1 | 0.078 | 0.265 | 3.39× |
| 10 | 5 | 0.325 | 1.430 | 4.40× |
| 10 | 10 | 0.643 | 2.957 | 4.60× |
| 100 | 1 | 0.105 | 0.265 | 2.52× |
| 100 | 5 | 0.306 | 1.433 | 4.69× |
| 100 | 10 | 0.689 | 2.970 | 4.31× |
| 1,000 | 1 | 0.427 | 0.307 | 0.72× |
| 1,000 | 5 | 0.612 | 1.683 | 2.75× |
| 1,000 | 10 | 1.167 | 3.430 | 2.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
