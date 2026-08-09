# RogersSatchell benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.48M | 0.035 | 28.47M | nan | — | — |
| 10,000 | 0.303 | 32.99M | 0.305 | 32.77M | nan | — | — |
| 100,000 | 2.953 | 33.86M | 2.844 | 35.17M | nan | — | — |
| 1,000,000 | 30.936 | 32.33M | 29.241 | 34.20M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.169 | nan | — |
| 1 | 5 | 0.453 | nan | — |
| 1 | 10 | 0.526 | nan | — |
| 10 | 1 | 0.057 | nan | — |
| 10 | 5 | 0.256 | nan | — |
| 10 | 10 | 0.505 | nan | — |
| 100 | 1 | 0.056 | nan | — |
| 100 | 5 | 0.244 | nan | — |
| 100 | 10 | 0.537 | nan | — |
| 1,000 | 1 | 0.084 | nan | — |
| 1,000 | 5 | 0.249 | nan | — |
| 1,000 | 10 | 0.565 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
