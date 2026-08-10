# FairValueGap benchmark (`smartmoneyconcepts.smc.fvg` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.68M | 0.017 | 57.15M | 3.516 | 167.62× | 200.92× |
| 10,000 | 0.134 | 74.35M | 0.115 | 87.33M | 10.315 | 76.69× | 90.08× |
| 100,000 | 1.327 | 75.38M | 1.072 | 93.25M | 74.036 | 55.81× | 69.04× |
| 1,000,000 | 25.689 | 38.93M | 23.030 | 43.42M | 733.860 | 28.57× | 31.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 3.010 | 36.49× |
| 1 | 5 | 0.386 | 15.526 | 40.22× |
| 1 | 10 | 0.573 | 28.789 | 50.21× |
| 10 | 1 | 0.065 | 2.805 | 43.17× |
| 10 | 5 | 0.296 | 14.588 | 49.32× |
| 10 | 10 | 0.652 | 31.302 | 47.99× |
| 100 | 1 | 0.068 | 2.967 | 43.33× |
| 100 | 5 | 0.284 | 15.872 | 55.80× |
| 100 | 10 | 0.598 | 32.465 | 54.27× |
| 1,000 | 1 | 0.085 | 3.642 | 42.92× |
| 1,000 | 5 | 0.294 | 19.088 | 64.93× |
| 1,000 | 10 | 0.587 | 39.025 | 66.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
