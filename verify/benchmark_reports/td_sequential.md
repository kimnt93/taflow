# TomDeMarkSequential benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 122.27M | 0.007 | 135.32M | nan | — | — |
| 10,000 | 0.065 | 153.57M | 0.063 | 159.60M | nan | — | — |
| 100,000 | 0.612 | 163.35M | 0.582 | 171.74M | nan | — | — |
| 1,000,000 | 6.868 | 145.60M | 6.201 | 161.25M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | nan | — |
| 1 | 5 | 0.287 | nan | — |
| 1 | 10 | 0.444 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.198 | nan | — |
| 10 | 10 | 0.448 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.199 | nan | — |
| 100 | 10 | 0.435 | nan | — |
| 1,000 | 1 | 0.050 | nan | — |
| 1,000 | 5 | 0.270 | nan | — |
| 1,000 | 10 | 0.556 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
