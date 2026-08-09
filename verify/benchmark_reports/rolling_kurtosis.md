# RollingKurtosis benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.62M | 0.019 | 53.69M | nan | — | — |
| 10,000 | 0.162 | 61.87M | 0.167 | 59.87M | nan | — | — |
| 100,000 | 1.586 | 63.05M | 1.627 | 61.48M | nan | — | — |
| 1,000,000 | 17.081 | 58.55M | 17.537 | 57.02M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | nan | — |
| 1 | 5 | 0.256 | nan | — |
| 1 | 10 | 0.477 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.201 | nan | — |
| 10 | 10 | 0.454 | nan | — |
| 100 | 1 | 0.046 | nan | — |
| 100 | 5 | 0.216 | nan | — |
| 100 | 10 | 0.456 | nan | — |
| 1,000 | 1 | 0.067 | nan | — |
| 1,000 | 5 | 0.233 | nan | — |
| 1,000 | 10 | 0.522 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
