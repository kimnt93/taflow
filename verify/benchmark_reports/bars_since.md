# BarsSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 213.60M | 0.004 | 263.88M | nan | — | — |
| 10,000 | 0.028 | 359.44M | 0.024 | 411.07M | nan | — | — |
| 100,000 | 0.243 | 412.19M | 0.219 | 456.04M | nan | — | — |
| 1,000,000 | 2.554 | 391.49M | 2.255 | 443.48M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | nan | — |
| 1 | 5 | 0.273 | nan | — |
| 1 | 10 | 0.466 | nan | — |
| 10 | 1 | 0.044 | nan | — |
| 10 | 5 | 0.223 | nan | — |
| 10 | 10 | 0.496 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.220 | nan | — |
| 100 | 10 | 0.490 | nan | — |
| 1,000 | 1 | 0.048 | nan | — |
| 1,000 | 5 | 0.228 | nan | — |
| 1,000 | 10 | 0.494 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
