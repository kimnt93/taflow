# Hurst benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.39M | 0.059 | 17.09M | nan | — | — |
| 10,000 | 0.602 | 16.61M | 0.551 | 18.15M | nan | — | — |
| 100,000 | 5.851 | 17.09M | 5.657 | 17.68M | nan | — | — |
| 1,000,000 | 56.786 | 17.61M | 68.313 | 14.64M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | nan | — |
| 1 | 5 | 0.302 | nan | — |
| 1 | 10 | 0.489 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.229 | nan | — |
| 10 | 10 | 0.466 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.233 | nan | — |
| 100 | 10 | 0.479 | nan | — |
| 1,000 | 1 | 0.105 | nan | — |
| 1,000 | 5 | 0.271 | nan | — |
| 1,000 | 10 | 0.562 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
