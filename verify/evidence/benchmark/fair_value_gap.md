# FairValueGap benchmark (`smartmoneyconcepts.smc.fvg` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.86M | 0.018 | 56.03M | 3.424 | 150.20× | 191.86× |
| 10,000 | 0.122 | 81.64M | 0.106 | 93.92M | 10.106 | 82.50× | 94.91× |
| 100,000 | 1.242 | 80.53M | 1.091 | 91.67M | 73.319 | 59.04× | 67.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 2.813 | 29.09× |
| 1 | 5 | 0.336 | 14.486 | 43.07× |
| 1 | 10 | 0.542 | 28.551 | 52.67× |
| 10 | 1 | 0.065 | 2.811 | 43.14× |
| 10 | 5 | 0.256 | 14.717 | 57.55× |
| 10 | 10 | 0.552 | 29.584 | 53.64× |
| 100 | 1 | 0.070 | 2.876 | 40.92× |
| 100 | 5 | 0.258 | 14.759 | 57.14× |
| 100 | 10 | 0.585 | 29.484 | 50.37× |
| 1,000 | 1 | 0.076 | 3.438 | 44.99× |
| 1,000 | 5 | 0.311 | 18.687 | 60.04× |
| 1,000 | 10 | 0.605 | 37.758 | 62.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
