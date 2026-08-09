# Supertrend benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.72M | 0.020 | 50.20M | nan | — | — |
| 10,000 | 0.173 | 57.82M | 0.159 | 62.71M | nan | — | — |
| 100,000 | 1.774 | 56.36M | 1.616 | 61.89M | nan | — | — |
| 1,000,000 | 18.726 | 53.40M | 17.186 | 58.19M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | nan | — |
| 1 | 5 | 0.464 | nan | — |
| 1 | 10 | 0.518 | nan | — |
| 10 | 1 | 0.052 | nan | — |
| 10 | 5 | 0.250 | nan | — |
| 10 | 10 | 0.523 | nan | — |
| 100 | 1 | 0.055 | nan | — |
| 100 | 5 | 0.256 | nan | — |
| 100 | 10 | 0.545 | nan | — |
| 1,000 | 1 | 0.077 | nan | — |
| 1,000 | 5 | 0.266 | nan | — |
| 1,000 | 10 | 0.579 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
