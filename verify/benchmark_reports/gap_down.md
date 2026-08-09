# GapDown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.97M | 0.005 | 185.21M | nan | — | — |
| 10,000 | 0.031 | 327.72M | 0.033 | 307.02M | nan | — | — |
| 100,000 | 0.269 | 371.86M | 0.253 | 395.34M | nan | — | — |
| 1,000,000 | 3.142 | 318.27M | 2.621 | 381.54M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | nan | — |
| 1 | 5 | 0.442 | nan | — |
| 1 | 10 | 0.493 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.214 | nan | — |
| 10 | 10 | 0.463 | nan | — |
| 100 | 1 | 0.054 | nan | — |
| 100 | 5 | 0.223 | nan | — |
| 100 | 10 | 0.500 | nan | — |
| 1,000 | 1 | 0.050 | nan | — |
| 1,000 | 5 | 0.234 | nan | — |
| 1,000 | 10 | 0.509 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
