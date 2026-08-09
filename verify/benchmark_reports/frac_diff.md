# FracDiff benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.074 | 13.44M | 0.075 | 13.41M | nan | — | — |
| 10,000 | 7.911 | 1.26M | 7.518 | 1.33M | nan | — | — |
| 100,000 | 84.117 | 1.19M | 91.419 | 1.09M | nan | — | — |
| 1,000,000 | 852.222 | 1.17M | 840.601 | 1.19M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | nan | — |
| 1 | 5 | 0.390 | nan | — |
| 1 | 10 | 0.574 | nan | — |
| 10 | 1 | 0.056 | nan | — |
| 10 | 5 | 0.298 | nan | — |
| 10 | 10 | 0.576 | nan | — |
| 100 | 1 | 0.058 | nan | — |
| 100 | 5 | 0.262 | nan | — |
| 100 | 10 | 0.693 | nan | — |
| 1,000 | 1 | 0.129 | nan | — |
| 1,000 | 5 | 0.294 | nan | — |
| 1,000 | 10 | 0.607 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
