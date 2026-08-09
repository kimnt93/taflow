# RollingInformationRatio benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.44M | 0.034 | 29.01M | nan | — | — |
| 10,000 | 0.320 | 31.23M | 0.428 | 23.35M | nan | — | — |
| 100,000 | 3.343 | 29.92M | 3.181 | 31.44M | nan | — | — |
| 1,000,000 | 32.606 | 30.67M | 31.614 | 31.63M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.201 | nan | — |
| 1 | 5 | 0.316 | nan | — |
| 1 | 10 | 0.539 | nan | — |
| 10 | 1 | 0.053 | nan | — |
| 10 | 5 | 0.229 | nan | — |
| 10 | 10 | 0.478 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.232 | nan | — |
| 100 | 10 | 0.506 | nan | — |
| 1,000 | 1 | 0.084 | nan | — |
| 1,000 | 5 | 0.267 | nan | — |
| 1,000 | 10 | 0.567 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
