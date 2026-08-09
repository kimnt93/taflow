# EaseOfMovement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.04M | 0.007 | 151.13M | nan | — | — |
| 10,000 | 0.035 | 288.53M | 0.032 | 308.26M | nan | — | — |
| 100,000 | 0.364 | 274.72M | 0.320 | 312.77M | nan | — | — |
| 1,000,000 | 3.510 | 284.92M | 3.156 | 316.90M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | nan | — |
| 1 | 5 | 0.271 | nan | — |
| 1 | 10 | 0.536 | nan | — |
| 10 | 1 | 0.051 | nan | — |
| 10 | 5 | 0.239 | nan | — |
| 10 | 10 | 0.521 | nan | — |
| 100 | 1 | 0.052 | nan | — |
| 100 | 5 | 0.273 | nan | — |
| 100 | 10 | 0.526 | nan | — |
| 1,000 | 1 | 0.056 | nan | — |
| 1,000 | 5 | 0.261 | nan | — |
| 1,000 | 10 | 0.568 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
