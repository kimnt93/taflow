# HeikinAshi benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.81M | 0.015 | 66.30M | nan | — | — |
| 10,000 | 0.123 | 81.18M | 0.110 | 90.95M | nan | — | — |
| 100,000 | 1.324 | 75.54M | 1.083 | 92.32M | nan | — | — |
| 1,000,000 | 28.519 | 35.06M | 20.026 | 49.94M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | nan | — |
| 1 | 5 | 0.395 | nan | — |
| 1 | 10 | 0.448 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.194 | nan | — |
| 10 | 10 | 0.414 | nan | — |
| 100 | 1 | 0.054 | nan | — |
| 100 | 5 | 0.200 | nan | — |
| 100 | 10 | 0.426 | nan | — |
| 1,000 | 1 | 0.062 | nan | — |
| 1,000 | 5 | 0.308 | nan | — |
| 1,000 | 10 | 0.562 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
