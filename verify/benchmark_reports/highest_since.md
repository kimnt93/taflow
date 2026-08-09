# HighestSince benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.94M | 0.007 | 153.54M | nan | — | — |
| 10,000 | 0.040 | 250.93M | 0.038 | 265.35M | nan | — | — |
| 100,000 | 0.430 | 232.33M | 0.328 | 304.62M | nan | — | — |
| 1,000,000 | 3.838 | 260.56M | 3.452 | 289.70M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | nan | — |
| 1 | 5 | 0.380 | nan | — |
| 1 | 10 | 0.519 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.242 | nan | — |
| 10 | 10 | 0.478 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.216 | nan | — |
| 100 | 10 | 0.452 | nan | — |
| 1,000 | 1 | 0.052 | nan | — |
| 1,000 | 5 | 0.247 | nan | — |
| 1,000 | 10 | 0.499 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
