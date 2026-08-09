# SmoothedTrendChannel benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 64.80M | 0.014 | 69.62M | nan | — | — |
| 10,000 | 0.129 | 77.31M | 0.125 | 80.19M | nan | — | — |
| 100,000 | 1.284 | 77.90M | 1.377 | 72.62M | nan | — | — |
| 1,000,000 | 15.573 | 64.21M | 13.055 | 76.60M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | nan | — |
| 1 | 5 | 0.324 | nan | — |
| 1 | 10 | 0.458 | nan | — |
| 10 | 1 | 0.053 | nan | — |
| 10 | 5 | 0.217 | nan | — |
| 10 | 10 | 0.429 | nan | — |
| 100 | 1 | 0.051 | nan | — |
| 100 | 5 | 0.220 | nan | — |
| 100 | 10 | 0.466 | nan | — |
| 1,000 | 1 | 0.060 | nan | — |
| 1,000 | 5 | 0.289 | nan | — |
| 1,000 | 10 | 0.597 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
