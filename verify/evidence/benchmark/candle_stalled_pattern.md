# CandleStalledPattern benchmark (`CDLSTALLEDPATTERN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.24M | 0.023 | 44.42M | 0.044 | 1.92× | 1.97× |
| 10,000 | 0.172 | 58.22M | 0.170 | 58.95M | 0.164 | 0.96× | 0.97× |
| 100,000 | 1.858 | 53.81M | 1.631 | 61.32M | 1.351 | 0.73× | 0.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.120 | 1.17× |
| 1 | 5 | 0.473 | 0.489 | 1.03× |
| 1 | 10 | 0.567 | 0.948 | 1.67× |
| 10 | 1 | 0.053 | 0.088 | 1.67× |
| 10 | 5 | 0.272 | 0.463 | 1.70× |
| 10 | 10 | 0.619 | 0.940 | 1.52× |
| 100 | 1 | 0.055 | 0.096 | 1.75× |
| 100 | 5 | 0.294 | 0.452 | 1.54× |
| 100 | 10 | 0.617 | 1.031 | 1.67× |
| 1,000 | 1 | 0.072 | 0.109 | 1.51× |
| 1,000 | 5 | 0.307 | 0.527 | 1.72× |
| 1,000 | 10 | 0.600 | 1.144 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
