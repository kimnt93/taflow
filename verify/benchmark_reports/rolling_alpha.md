# RollingAlpha benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.041 | 24.18M | 0.040 | 24.96M | nan | — | — |
| 10,000 | 0.379 | 26.41M | 0.386 | 25.92M | nan | — | — |
| 100,000 | 3.890 | 25.70M | 3.750 | 26.67M | nan | — | — |
| 1,000,000 | 37.233 | 26.86M | 38.356 | 26.07M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | nan | — |
| 1 | 5 | 0.389 | nan | — |
| 1 | 10 | 0.479 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.234 | nan | — |
| 10 | 10 | 0.463 | nan | — |
| 100 | 1 | 0.054 | nan | — |
| 100 | 5 | 0.264 | nan | — |
| 100 | 10 | 0.538 | nan | — |
| 1,000 | 1 | 0.085 | nan | — |
| 1,000 | 5 | 0.263 | nan | — |
| 1,000 | 10 | 0.542 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
