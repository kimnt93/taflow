# HigherHigh benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 135.13M | 0.006 | 168.79M | nan | — | — |
| 10,000 | 0.033 | 299.39M | 0.030 | 336.71M | nan | — | — |
| 100,000 | 0.388 | 257.63M | 0.251 | 397.95M | nan | — | — |
| 1,000,000 | 3.352 | 298.33M | 3.352 | 298.29M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | nan | — |
| 1 | 5 | 0.319 | nan | — |
| 1 | 10 | 0.477 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.234 | nan | — |
| 10 | 10 | 0.496 | nan | — |
| 100 | 1 | 0.046 | nan | — |
| 100 | 5 | 0.255 | nan | — |
| 100 | 10 | 0.482 | nan | — |
| 1,000 | 1 | 0.049 | nan | — |
| 1,000 | 5 | 0.243 | nan | — |
| 1,000 | 10 | 0.532 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
