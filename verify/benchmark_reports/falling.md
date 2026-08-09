# Falling benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.28M | 0.007 | 138.94M | nan | — | — |
| 10,000 | 0.050 | 199.53M | 0.048 | 207.07M | nan | — | — |
| 100,000 | 0.540 | 185.07M | 0.447 | 223.69M | nan | — | — |
| 1,000,000 | 5.453 | 183.39M | 4.855 | 205.99M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | nan | — |
| 1 | 5 | 0.273 | nan | — |
| 1 | 10 | 0.534 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.240 | nan | — |
| 10 | 10 | 0.506 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.238 | nan | — |
| 100 | 10 | 0.530 | nan | — |
| 1,000 | 1 | 0.053 | nan | — |
| 1,000 | 5 | 0.267 | nan | — |
| 1,000 | 10 | 0.524 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
