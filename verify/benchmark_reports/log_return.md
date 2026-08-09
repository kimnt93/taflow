# LogReturn benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.25M | 0.010 | 97.35M | nan | — | — |
| 10,000 | 0.081 | 124.05M | 0.076 | 132.05M | nan | — | — |
| 100,000 | 0.752 | 132.92M | 0.763 | 131.00M | nan | — | — |
| 1,000,000 | 7.617 | 131.29M | 7.196 | 138.96M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | nan | — |
| 1 | 5 | 0.448 | nan | — |
| 1 | 10 | 0.468 | nan | — |
| 10 | 1 | 0.043 | nan | — |
| 10 | 5 | 0.208 | nan | — |
| 10 | 10 | 0.452 | nan | — |
| 100 | 1 | 0.045 | nan | — |
| 100 | 5 | 0.243 | nan | — |
| 100 | 10 | 0.465 | nan | — |
| 1,000 | 1 | 0.057 | nan | — |
| 1,000 | 5 | 0.220 | nan | — |
| 1,000 | 10 | 0.469 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
