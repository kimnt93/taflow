# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.15M | 0.007 | 136.10M | 0.031 | 2.99× | 4.23× |
| 10,000 | 0.076 | 131.92M | 0.069 | 145.92M | 0.082 | 1.08× | 1.19× |
| 100,000 | 0.740 | 135.20M | 0.728 | 137.27M | 0.564 | 0.76× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.122 | 1.13× |
| 1 | 5 | 0.336 | 0.440 | 1.31× |
| 1 | 10 | 0.590 | 1.031 | 1.75× |
| 10 | 1 | 0.061 | 0.086 | 1.41× |
| 10 | 5 | 0.249 | 0.430 | 1.72× |
| 10 | 10 | 0.516 | 0.876 | 1.70× |
| 100 | 1 | 0.056 | 0.089 | 1.60× |
| 100 | 5 | 0.252 | 0.442 | 1.75× |
| 100 | 10 | 0.551 | 0.871 | 1.58× |
| 1,000 | 1 | 0.067 | 0.094 | 1.39× |
| 1,000 | 5 | 0.252 | 0.449 | 1.78× |
| 1,000 | 10 | 0.557 | 0.963 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
