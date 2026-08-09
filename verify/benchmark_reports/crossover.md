# Crossover benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.91M | 0.006 | 160.06M | nan | — | — |
| 10,000 | 0.033 | 300.85M | 0.030 | 338.18M | nan | — | — |
| 100,000 | 0.295 | 339.01M | 0.293 | 341.32M | nan | — | — |
| 1,000,000 | 3.367 | 296.96M | 3.073 | 325.40M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | nan | — |
| 1 | 5 | 0.276 | nan | — |
| 1 | 10 | 0.452 | nan | — |
| 10 | 1 | 0.045 | nan | — |
| 10 | 5 | 0.219 | nan | — |
| 10 | 10 | 0.452 | nan | — |
| 100 | 1 | 0.048 | nan | — |
| 100 | 5 | 0.220 | nan | — |
| 100 | 10 | 0.444 | nan | — |
| 1,000 | 1 | 0.053 | nan | — |
| 1,000 | 5 | 0.236 | nan | — |
| 1,000 | 10 | 0.500 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
