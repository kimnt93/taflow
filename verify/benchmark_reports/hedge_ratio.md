# HedgeRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.03M | 0.037 | 27.33M | nan | — | — |
| 10,000 | 0.378 | 26.42M | 0.349 | 28.64M | nan | — | — |
| 100,000 | 3.532 | 28.31M | 3.564 | 28.06M | nan | — | — |
| 1,000,000 | 35.155 | 28.45M | 35.120 | 28.47M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | nan | — |
| 1 | 5 | 0.319 | nan | — |
| 1 | 10 | 0.493 | nan | — |
| 10 | 1 | 0.044 | nan | — |
| 10 | 5 | 0.213 | nan | — |
| 10 | 10 | 0.473 | nan | — |
| 100 | 1 | 0.055 | nan | — |
| 100 | 5 | 0.237 | nan | — |
| 100 | 10 | 0.501 | nan | — |
| 1,000 | 1 | 0.086 | nan | — |
| 1,000 | 5 | 0.236 | nan | — |
| 1,000 | 10 | 0.526 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
