# RollSpread benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.38M | 0.047 | 21.46M | nan | — | — |
| 10,000 | 0.447 | 22.36M | 0.442 | 22.62M | nan | — | — |
| 100,000 | 4.654 | 21.49M | 4.503 | 22.21M | nan | — | — |
| 1,000,000 | 45.129 | 22.16M | 45.008 | 22.22M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.150 | nan | — |
| 1 | 5 | 0.348 | nan | — |
| 1 | 10 | 0.481 | nan | — |
| 10 | 1 | 0.051 | nan | — |
| 10 | 5 | 0.209 | nan | — |
| 10 | 10 | 0.471 | nan | — |
| 100 | 1 | 0.057 | nan | — |
| 100 | 5 | 0.245 | nan | — |
| 100 | 10 | 0.530 | nan | — |
| 1,000 | 1 | 0.097 | nan | — |
| 1,000 | 5 | 0.245 | nan | — |
| 1,000 | 10 | 0.524 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
