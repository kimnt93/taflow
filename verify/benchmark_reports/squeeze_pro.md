# SqueezePro benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.72M | 0.047 | 21.30M | nan | — | — |
| 10,000 | 0.439 | 22.77M | 0.406 | 24.65M | nan | — | — |
| 100,000 | 4.354 | 22.97M | 4.224 | 23.67M | nan | — | — |
| 1,000,000 | 68.519 | 14.59M | 52.372 | 19.09M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | nan | — |
| 1 | 5 | 0.281 | nan | — |
| 1 | 10 | 0.495 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.234 | nan | — |
| 10 | 10 | 0.512 | nan | — |
| 100 | 1 | 0.059 | nan | — |
| 100 | 5 | 0.250 | nan | — |
| 100 | 10 | 0.535 | nan | — |
| 1,000 | 1 | 0.098 | nan | — |
| 1,000 | 5 | 0.261 | nan | — |
| 1,000 | 10 | 0.585 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
