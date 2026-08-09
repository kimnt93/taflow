# Squeeze benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 18.00M | 0.044 | 22.67M | nan | — | — |
| 10,000 | 0.511 | 19.56M | 0.417 | 23.96M | nan | — | — |
| 100,000 | 5.167 | 19.35M | 3.793 | 26.37M | nan | — | — |
| 1,000,000 | 65.909 | 15.17M | 47.165 | 21.20M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | nan | — |
| 1 | 5 | 0.458 | nan | — |
| 1 | 10 | 0.523 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.237 | nan | — |
| 10 | 10 | 0.516 | nan | — |
| 100 | 1 | 0.057 | nan | — |
| 100 | 5 | 0.240 | nan | — |
| 100 | 10 | 0.533 | nan | — |
| 1,000 | 1 | 0.092 | nan | — |
| 1,000 | 5 | 0.251 | nan | — |
| 1,000 | 10 | 0.560 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
