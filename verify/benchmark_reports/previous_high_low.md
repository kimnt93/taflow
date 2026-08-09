# PreviousHighLow benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 63.03M | 0.013 | 74.55M | nan | — | — |
| 10,000 | 0.101 | 98.55M | 0.102 | 97.87M | nan | — | — |
| 100,000 | 1.049 | 95.31M | 0.879 | 113.75M | nan | — | — |
| 1,000,000 | 21.157 | 47.27M | 9.278 | 107.79M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | nan | — |
| 1 | 5 | 0.309 | nan | — |
| 1 | 10 | 0.486 | nan | — |
| 10 | 1 | 0.051 | nan | — |
| 10 | 5 | 0.239 | nan | — |
| 10 | 10 | 0.490 | nan | — |
| 100 | 1 | 0.048 | nan | — |
| 100 | 5 | 0.244 | nan | — |
| 100 | 10 | 0.524 | nan | — |
| 1,000 | 1 | 0.065 | nan | — |
| 1,000 | 5 | 0.262 | nan | — |
| 1,000 | 10 | 0.540 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
