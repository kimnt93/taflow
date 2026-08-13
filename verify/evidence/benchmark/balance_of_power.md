# BalanceOfPower benchmark (`BOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.67M | 0.039 | 25.40M | 0.028 | 0.61× | 0.71× |
| 10,000 | 0.287 | 34.80M | 0.296 | 33.79M | 0.039 | 0.13× | 0.13× |
| 100,000 | 2.665 | 37.52M | 2.649 | 37.75M | 0.125 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.124 | 1.25× |
| 1 | 5 | 0.467 | 0.444 | 0.95× |
| 1 | 10 | 0.662 | 0.872 | 1.32× |
| 10 | 1 | 0.071 | 0.087 | 1.23× |
| 10 | 5 | 0.309 | 0.411 | 1.33× |
| 10 | 10 | 0.615 | 0.906 | 1.47× |
| 100 | 1 | 0.073 | 0.086 | 1.17× |
| 100 | 5 | 0.327 | 0.431 | 1.32× |
| 100 | 10 | 0.671 | 1.017 | 1.52× |
| 1,000 | 1 | 0.112 | 0.088 | 0.78× |
| 1,000 | 5 | 0.337 | 0.425 | 1.26× |
| 1,000 | 10 | 0.651 | 0.886 | 1.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
