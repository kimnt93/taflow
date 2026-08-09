# KnowSureThing benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.64M | 0.019 | 53.82M | nan | — | — |
| 10,000 | 0.156 | 64.14M | 0.156 | 63.97M | nan | — | — |
| 100,000 | 1.501 | 66.63M | 1.458 | 68.57M | nan | — | — |
| 1,000,000 | 15.495 | 64.54M | 14.819 | 67.48M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | nan | — |
| 1 | 5 | 0.280 | nan | — |
| 1 | 10 | 0.472 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.221 | nan | — |
| 10 | 10 | 0.466 | nan | — |
| 100 | 1 | 0.052 | nan | — |
| 100 | 5 | 0.242 | nan | — |
| 100 | 10 | 0.495 | nan | — |
| 1,000 | 1 | 0.062 | nan | — |
| 1,000 | 5 | 0.227 | nan | — |
| 1,000 | 10 | 0.507 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
