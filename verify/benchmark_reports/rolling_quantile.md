# RollingQuantile benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.67M | 0.049 | 20.61M | nan | — | — |
| 10,000 | 0.500 | 20.00M | 0.484 | 20.64M | nan | — | — |
| 100,000 | 4.675 | 21.39M | 4.490 | 22.27M | nan | — | — |
| 1,000,000 | 46.887 | 21.33M | 45.346 | 22.05M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | nan | — |
| 1 | 5 | 0.378 | nan | — |
| 1 | 10 | 0.449 | nan | — |
| 10 | 1 | 0.053 | nan | — |
| 10 | 5 | 0.208 | nan | — |
| 10 | 10 | 0.426 | nan | — |
| 100 | 1 | 0.053 | nan | — |
| 100 | 5 | 0.223 | nan | — |
| 100 | 10 | 0.521 | nan | — |
| 1,000 | 1 | 0.102 | nan | — |
| 1,000 | 5 | 0.224 | nan | — |
| 1,000 | 10 | 0.508 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
