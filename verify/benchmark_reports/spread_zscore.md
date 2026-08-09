# SpreadZScore benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.099 | 10.07M | 0.099 | 10.07M | nan | — | — |
| 10,000 | 0.922 | 10.84M | 0.948 | 10.55M | nan | — | — |
| 100,000 | 9.248 | 10.81M | 9.237 | 10.83M | nan | — | — |
| 1,000,000 | 94.153 | 10.62M | 93.993 | 10.64M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | nan | — |
| 1 | 5 | 0.385 | nan | — |
| 1 | 10 | 0.544 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.236 | nan | — |
| 10 | 10 | 0.524 | nan | — |
| 100 | 1 | 0.062 | nan | — |
| 100 | 5 | 0.287 | nan | — |
| 100 | 10 | 0.601 | nan | — |
| 1,000 | 1 | 0.154 | nan | — |
| 1,000 | 5 | 0.305 | nan | — |
| 1,000 | 10 | 0.556 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
