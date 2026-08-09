# Vortex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.85M | 0.015 | 64.89M | nan | — | — |
| 10,000 | 0.116 | 86.36M | 0.107 | 93.54M | nan | — | — |
| 100,000 | 1.068 | 93.63M | 1.015 | 98.54M | nan | — | — |
| 1,000,000 | 11.051 | 90.49M | 10.477 | 95.45M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | nan | — |
| 1 | 5 | 0.362 | nan | — |
| 1 | 10 | 0.503 | nan | — |
| 10 | 1 | 0.056 | nan | — |
| 10 | 5 | 0.236 | nan | — |
| 10 | 10 | 0.489 | nan | — |
| 100 | 1 | 0.054 | nan | — |
| 100 | 5 | 0.233 | nan | — |
| 100 | 10 | 0.499 | nan | — |
| 1,000 | 1 | 0.061 | nan | — |
| 1,000 | 5 | 0.266 | nan | — |
| 1,000 | 10 | 0.534 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
