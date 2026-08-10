# AdaptiveCycle benchmark (`AdaptiveCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.067 | 14.89M | 0.065 | 15.47M | 0.208 | 3.10× | 3.22× |
| 10,000 | 0.720 | 13.88M | 0.613 | 16.32M | 0.988 | 1.37× | 1.61× |
| 100,000 | 6.257 | 15.98M | 7.674 | 13.03M | 9.959 | 1.59× | 1.30× |
| 1,000,000 | 63.746 | 15.69M | 61.787 | 16.18M | 104.758 | 1.64× | 1.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.361 | 4.37× |
| 1 | 5 | 0.330 | 0.849 | 2.57× |
| 1 | 10 | 0.486 | 1.910 | 3.93× |
| 10 | 1 | 0.055 | 0.167 | 3.04× |
| 10 | 5 | 0.239 | 0.800 | 3.34× |
| 10 | 10 | 0.522 | 2.017 | 3.86× |
| 100 | 1 | 0.061 | 0.174 | 2.88× |
| 100 | 5 | 0.267 | 0.851 | 3.18× |
| 100 | 10 | 0.490 | 1.898 | 3.87× |
| 1,000 | 1 | 0.122 | 0.275 | 2.25× |
| 1,000 | 5 | 0.324 | 1.662 | 5.13× |
| 1,000 | 10 | 0.664 | 2.913 | 4.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
