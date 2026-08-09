# Lag benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.82M | 0.006 | 176.72M | nan | — | — |
| 10,000 | 0.035 | 287.89M | 0.034 | 297.37M | nan | — | — |
| 100,000 | 0.442 | 226.26M | 0.306 | 326.82M | nan | — | — |
| 1,000,000 | 3.652 | 273.85M | 3.159 | 316.53M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | nan | — |
| 1 | 5 | 0.317 | nan | — |
| 1 | 10 | 0.520 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.253 | nan | — |
| 10 | 10 | 0.524 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.235 | nan | — |
| 100 | 10 | 0.533 | nan | — |
| 1,000 | 1 | 0.056 | nan | — |
| 1,000 | 5 | 0.260 | nan | — |
| 1,000 | 10 | 0.557 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
