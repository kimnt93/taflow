# RollingMedian benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.55M | 0.038 | 26.17M | nan | — | — |
| 10,000 | 0.395 | 25.30M | 0.388 | 25.75M | nan | — | — |
| 100,000 | 3.803 | 26.30M | 3.899 | 25.65M | nan | — | — |
| 1,000,000 | 39.388 | 25.39M | 39.371 | 25.40M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | nan | — |
| 1 | 5 | 0.316 | nan | — |
| 1 | 10 | 0.499 | nan | — |
| 10 | 1 | 0.051 | nan | — |
| 10 | 5 | 0.230 | nan | — |
| 10 | 10 | 0.467 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.212 | nan | — |
| 100 | 10 | 0.482 | nan | — |
| 1,000 | 1 | 0.091 | nan | — |
| 1,000 | 5 | 0.221 | nan | — |
| 1,000 | 10 | 0.529 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
