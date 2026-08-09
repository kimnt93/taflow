# RollingSkew benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.66M | 0.035 | 28.92M | nan | — | — |
| 10,000 | 0.351 | 28.52M | 0.329 | 30.41M | nan | — | — |
| 100,000 | 3.416 | 29.27M | 3.329 | 30.04M | nan | — | — |
| 1,000,000 | 34.693 | 28.82M | 33.170 | 30.15M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | nan | — |
| 1 | 5 | 0.306 | nan | — |
| 1 | 10 | 0.484 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.228 | nan | — |
| 10 | 10 | 0.484 | nan | — |
| 100 | 1 | 0.052 | nan | — |
| 100 | 5 | 0.233 | nan | — |
| 100 | 10 | 0.495 | nan | — |
| 1,000 | 1 | 0.077 | nan | — |
| 1,000 | 5 | 0.241 | nan | — |
| 1,000 | 10 | 0.510 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
