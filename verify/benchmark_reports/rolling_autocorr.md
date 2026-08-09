# RollingAutocorr benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.16M | 0.051 | 19.51M | nan | — | — |
| 10,000 | 0.507 | 19.72M | 0.522 | 19.14M | nan | — | — |
| 100,000 | 5.137 | 19.47M | 5.036 | 19.86M | nan | — | — |
| 1,000,000 | 50.758 | 19.70M | 51.431 | 19.44M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | nan | — |
| 1 | 5 | 0.358 | nan | — |
| 1 | 10 | 0.468 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.218 | nan | — |
| 10 | 10 | 0.432 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.212 | nan | — |
| 100 | 10 | 0.483 | nan | — |
| 1,000 | 1 | 0.101 | nan | — |
| 1,000 | 5 | 0.237 | nan | — |
| 1,000 | 10 | 0.515 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
