# Liquidity benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.45M | 0.039 | 25.79M | nan | — | — |
| 10,000 | 0.384 | 26.02M | 0.375 | 26.64M | nan | — | — |
| 100,000 | 4.230 | 23.64M | 4.314 | 23.18M | nan | — | — |
| 1,000,000 | 55.476 | 18.03M | 44.406 | 22.52M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | nan | — |
| 1 | 5 | 0.341 | nan | — |
| 1 | 10 | 0.507 | nan | — |
| 10 | 1 | 0.051 | nan | — |
| 10 | 5 | 0.234 | nan | — |
| 10 | 10 | 0.511 | nan | — |
| 100 | 1 | 0.062 | nan | — |
| 100 | 5 | 0.232 | nan | — |
| 100 | 10 | 0.504 | nan | — |
| 1,000 | 1 | 0.091 | nan | — |
| 1,000 | 5 | 0.236 | nan | — |
| 1,000 | 10 | 0.539 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
