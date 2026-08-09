# KalmanHedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.88M | 0.018 | 54.95M | nan | — | — |
| 10,000 | 0.155 | 64.34M | 0.152 | 65.82M | nan | — | — |
| 100,000 | 1.495 | 66.87M | 1.482 | 67.45M | nan | — | — |
| 1,000,000 | 15.378 | 65.03M | 14.848 | 67.35M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | nan | — |
| 1 | 5 | 0.313 | nan | — |
| 1 | 10 | 0.504 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.217 | nan | — |
| 10 | 10 | 0.473 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.227 | nan | — |
| 100 | 10 | 0.499 | nan | — |
| 1,000 | 1 | 0.091 | nan | — |
| 1,000 | 5 | 0.288 | nan | — |
| 1,000 | 10 | 0.535 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
