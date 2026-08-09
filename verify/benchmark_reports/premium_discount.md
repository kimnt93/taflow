# PremiumDiscount benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.25M | 0.022 | 45.18M | nan | — | — |
| 10,000 | 0.270 | 36.97M | 0.309 | 32.40M | nan | — | — |
| 100,000 | 2.781 | 35.96M | 2.722 | 36.73M | nan | — | — |
| 1,000,000 | 28.856 | 34.65M | 28.417 | 35.19M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | nan | — |
| 1 | 5 | 0.295 | nan | — |
| 1 | 10 | 0.459 | nan | — |
| 10 | 1 | 0.048 | nan | — |
| 10 | 5 | 0.204 | nan | — |
| 10 | 10 | 0.443 | nan | — |
| 100 | 1 | 0.054 | nan | — |
| 100 | 5 | 0.229 | nan | — |
| 100 | 10 | 0.476 | nan | — |
| 1,000 | 1 | 0.081 | nan | — |
| 1,000 | 5 | 0.369 | nan | — |
| 1,000 | 10 | 0.735 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
