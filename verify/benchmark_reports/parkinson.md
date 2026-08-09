# Parkinson benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.80M | 0.018 | 54.85M | nan | — | — |
| 10,000 | 0.150 | 66.45M | 0.146 | 68.60M | nan | — | — |
| 100,000 | 1.547 | 64.64M | 1.486 | 67.28M | nan | — | — |
| 1,000,000 | 21.648 | 46.19M | 14.872 | 67.24M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | nan | — |
| 1 | 5 | 0.331 | nan | — |
| 1 | 10 | 0.552 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.250 | nan | — |
| 10 | 10 | 0.500 | nan | — |
| 100 | 1 | 0.056 | nan | — |
| 100 | 5 | 0.252 | nan | — |
| 100 | 10 | 0.603 | nan | — |
| 1,000 | 1 | 0.087 | nan | — |
| 1,000 | 5 | 0.271 | nan | — |
| 1,000 | 10 | 0.539 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
