# RollingRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.61M | 0.019 | 53.57M | nan | — | — |
| 10,000 | 0.165 | 60.66M | 0.164 | 60.81M | nan | — | — |
| 100,000 | 1.670 | 59.89M | 1.613 | 61.98M | nan | — | — |
| 1,000,000 | 17.346 | 57.65M | 16.660 | 60.02M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | nan | — |
| 1 | 5 | 0.364 | nan | — |
| 1 | 10 | 0.500 | nan | — |
| 10 | 1 | 0.048 | nan | — |
| 10 | 5 | 0.206 | nan | — |
| 10 | 10 | 0.445 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.225 | nan | — |
| 100 | 10 | 0.445 | nan | — |
| 1,000 | 1 | 0.067 | nan | — |
| 1,000 | 5 | 0.239 | nan | — |
| 1,000 | 10 | 0.512 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
