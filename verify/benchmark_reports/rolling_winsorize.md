# RollingWinsorize benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.88M | 0.053 | 18.86M | nan | — | — |
| 10,000 | 0.559 | 17.90M | 0.554 | 18.06M | nan | — | — |
| 100,000 | 5.849 | 17.10M | 5.881 | 17.00M | nan | — | — |
| 1,000,000 | 59.274 | 16.87M | 58.322 | 17.15M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | nan | — |
| 1 | 5 | 0.262 | nan | — |
| 1 | 10 | 0.515 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.248 | nan | — |
| 10 | 10 | 0.524 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.253 | nan | — |
| 100 | 10 | 0.555 | nan | — |
| 1,000 | 1 | 0.109 | nan | — |
| 1,000 | 5 | 0.299 | nan | — |
| 1,000 | 10 | 0.560 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
