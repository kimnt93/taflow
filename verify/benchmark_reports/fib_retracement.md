# FibonacciRetracement benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 42.26M | 0.021 | 47.46M | nan | — | — |
| 10,000 | 0.287 | 34.81M | 0.271 | 36.84M | nan | — | — |
| 100,000 | 2.904 | 34.44M | 2.814 | 35.54M | nan | — | — |
| 1,000,000 | 61.286 | 16.32M | 39.589 | 25.26M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | nan | — |
| 1 | 5 | 0.271 | nan | — |
| 1 | 10 | 0.482 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.203 | nan | — |
| 10 | 10 | 0.438 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.247 | nan | — |
| 100 | 10 | 0.488 | nan | — |
| 1,000 | 1 | 0.081 | nan | — |
| 1,000 | 5 | 0.375 | nan | — |
| 1,000 | 10 | 0.773 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
