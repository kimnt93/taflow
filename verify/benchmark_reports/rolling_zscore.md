# RollingZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 31.38M | 0.029 | 34.21M | nan | — | — |
| 10,000 | 0.284 | 35.27M | 0.283 | 35.33M | nan | — | — |
| 100,000 | 2.867 | 34.88M | 2.769 | 36.12M | nan | — | — |
| 1,000,000 | 28.666 | 34.88M | 26.911 | 37.16M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.057 | nan | — |
| 1 | 5 | 0.326 | nan | — |
| 1 | 10 | 0.462 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.204 | nan | — |
| 10 | 10 | 0.439 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.232 | nan | — |
| 100 | 10 | 0.458 | nan | — |
| 1,000 | 1 | 0.078 | nan | — |
| 1,000 | 5 | 0.226 | nan | — |
| 1,000 | 10 | 0.511 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
