# HullMovingAverage benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.51M | 0.023 | 43.55M | nan | — | — |
| 10,000 | 0.209 | 47.92M | 0.202 | 49.59M | nan | — | — |
| 100,000 | 2.150 | 46.51M | 2.020 | 49.51M | nan | — | — |
| 1,000,000 | 20.536 | 48.70M | 20.341 | 49.16M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | nan | — |
| 1 | 5 | 0.335 | nan | — |
| 1 | 10 | 0.461 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.207 | nan | — |
| 10 | 10 | 0.452 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.214 | nan | — |
| 100 | 10 | 0.477 | nan | — |
| 1,000 | 1 | 0.066 | nan | — |
| 1,000 | 5 | 0.242 | nan | — |
| 1,000 | 10 | 0.486 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
