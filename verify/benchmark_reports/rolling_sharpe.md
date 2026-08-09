# RollingSharpe benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.44M | 0.029 | 34.28M | nan | — | — |
| 10,000 | 0.273 | 36.67M | 0.272 | 36.82M | nan | — | — |
| 100,000 | 3.127 | 31.98M | 2.773 | 36.06M | nan | — | — |
| 1,000,000 | 29.055 | 34.42M | 26.994 | 37.05M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | nan | — |
| 1 | 5 | 0.343 | nan | — |
| 1 | 10 | 0.469 | nan | — |
| 10 | 1 | 0.045 | nan | — |
| 10 | 5 | 0.226 | nan | — |
| 10 | 10 | 0.462 | nan | — |
| 100 | 1 | 0.048 | nan | — |
| 100 | 5 | 0.237 | nan | — |
| 100 | 10 | 0.485 | nan | — |
| 1,000 | 1 | 0.075 | nan | — |
| 1,000 | 5 | 0.262 | nan | — |
| 1,000 | 10 | 0.502 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
