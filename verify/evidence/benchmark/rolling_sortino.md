# RollingSortino benchmark (`SortinoRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.78M | 0.022 | 46.12M | 0.217 | 9.92× | 9.99× |
| 10,000 | 0.202 | 49.38M | 0.196 | 51.12M | 0.715 | 3.53× | 3.66× |
| 100,000 | 1.875 | 53.33M | 1.815 | 55.08M | 5.855 | 3.12× | 3.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.241 | 3.69× |
| 1 | 5 | 0.322 | 1.106 | 3.44× |
| 1 | 10 | 0.514 | 2.600 | 5.06× |
| 10 | 1 | 0.055 | 0.225 | 4.08× |
| 10 | 5 | 0.263 | 1.361 | 5.18× |
| 10 | 10 | 0.553 | 2.508 | 4.53× |
| 100 | 1 | 0.064 | 0.296 | 4.60× |
| 100 | 5 | 0.277 | 1.463 | 5.29× |
| 100 | 10 | 0.554 | 2.721 | 4.91× |
| 1,000 | 1 | 0.076 | 0.293 | 3.86× |
| 1,000 | 5 | 0.266 | 1.737 | 6.54× |
| 1,000 | 10 | 0.990 | 3.208 | 3.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
