# TimeSeriesRank benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.26M | 0.023 | 43.25M | nan | — | — |
| 10,000 | 0.181 | 55.21M | 0.172 | 58.29M | nan | — | — |
| 100,000 | 1.972 | 50.70M | 1.648 | 60.70M | nan | — | — |
| 1,000,000 | 19.314 | 51.78M | 17.317 | 57.75M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | nan | — |
| 1 | 5 | 0.315 | nan | — |
| 1 | 10 | 0.553 | nan | — |
| 10 | 1 | 0.056 | nan | — |
| 10 | 5 | 0.210 | nan | — |
| 10 | 10 | 0.464 | nan | — |
| 100 | 1 | 0.048 | nan | — |
| 100 | 5 | 0.225 | nan | — |
| 100 | 10 | 0.688 | nan | — |
| 1,000 | 1 | 0.067 | nan | — |
| 1,000 | 5 | 0.246 | nan | — |
| 1,000 | 10 | 0.510 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
