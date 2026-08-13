# BatPattern benchmark (`Bat` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.31M | 0.048 | 20.64M | 0.213 | 3.68× | 4.39× |
| 10,000 | 0.408 | 24.52M | 0.383 | 26.10M | 1.263 | 3.10× | 3.30× |
| 100,000 | 3.779 | 26.46M | 3.854 | 25.95M | 11.839 | 3.13× | 3.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.187 | 1.61× |
| 1 | 5 | 0.568 | 0.839 | 1.48× |
| 1 | 10 | 0.653 | 1.676 | 2.57× |
| 10 | 1 | 0.108 | 0.170 | 1.57× |
| 10 | 5 | 0.352 | 1.184 | 3.36× |
| 10 | 10 | 0.686 | 1.674 | 2.44× |
| 100 | 1 | 0.073 | 0.179 | 2.45× |
| 100 | 5 | 0.319 | 1.113 | 3.50× |
| 100 | 10 | 0.694 | 1.749 | 2.52× |
| 1,000 | 1 | 0.109 | 0.292 | 2.68× |
| 1,000 | 5 | 0.313 | 1.708 | 5.46× |
| 1,000 | 10 | 0.657 | 2.940 | 4.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
