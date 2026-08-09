# CloseToCloseSigma benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.025 | 40.08M | 0.023 | 43.63M | nan | — | — |
| 10,000 | 0.202 | 49.59M | 0.197 | 50.73M | nan | — | — |
| 100,000 | 2.031 | 49.23M | 1.949 | 51.30M | nan | — | — |
| 1,000,000 | 20.675 | 48.37M | 21.195 | 47.18M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | nan | — |
| 1 | 5 | 0.302 | nan | — |
| 1 | 10 | 0.501 | nan | — |
| 10 | 1 | 0.051 | nan | — |
| 10 | 5 | 0.298 | nan | — |
| 10 | 10 | 0.626 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.248 | nan | — |
| 100 | 10 | 0.550 | nan | — |
| 1,000 | 1 | 0.068 | nan | — |
| 1,000 | 5 | 0.256 | nan | — |
| 1,000 | 10 | 0.637 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
