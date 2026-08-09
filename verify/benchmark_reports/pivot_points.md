# PivotPoints benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 59.69M | 0.014 | 70.92M | nan | — | — |
| 10,000 | 0.116 | 85.98M | 0.102 | 98.06M | nan | — | — |
| 100,000 | 1.171 | 85.42M | 1.102 | 90.71M | nan | — | — |
| 1,000,000 | 42.725 | 23.41M | 31.591 | 31.65M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | nan | — |
| 1 | 5 | 0.315 | nan | — |
| 1 | 10 | 0.511 | nan | — |
| 10 | 1 | 0.074 | nan | — |
| 10 | 5 | 0.443 | nan | — |
| 10 | 10 | 0.541 | nan | — |
| 100 | 1 | 0.072 | nan | — |
| 100 | 5 | 0.271 | nan | — |
| 100 | 10 | 0.518 | nan | — |
| 1,000 | 1 | 0.059 | nan | — |
| 1,000 | 5 | 0.353 | nan | — |
| 1,000 | 10 | 0.633 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
