# FlagPennant benchmark (`FlagPennant` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 70.73M | 0.011 | 89.21M | 0.250 | 17.65× | 22.26× |
| 10,000 | 0.089 | 111.73M | 0.084 | 118.69M | 1.449 | 16.19× | 17.20× |
| 100,000 | 0.830 | 120.54M | 0.800 | 125.05M | 13.758 | 16.58× | 17.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.228 | 2.73× |
| 1 | 5 | 0.290 | 0.829 | 2.86× |
| 1 | 10 | 0.534 | 1.782 | 3.34× |
| 10 | 1 | 0.055 | 0.166 | 3.02× |
| 10 | 5 | 0.267 | 1.163 | 4.35× |
| 10 | 10 | 0.541 | 1.801 | 3.33× |
| 100 | 1 | 0.057 | 0.186 | 3.24× |
| 100 | 5 | 0.257 | 1.175 | 4.57× |
| 100 | 10 | 0.550 | 1.928 | 3.50× |
| 1,000 | 1 | 0.067 | 0.299 | 4.48× |
| 1,000 | 5 | 0.278 | 1.782 | 6.41× |
| 1,000 | 10 | 0.607 | 3.028 | 4.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
