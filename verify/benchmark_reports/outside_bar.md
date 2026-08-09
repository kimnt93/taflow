# OutsideBar benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.86M | 0.006 | 164.94M | nan | — | — |
| 10,000 | 0.034 | 297.77M | 0.030 | 330.32M | nan | — | — |
| 100,000 | 0.284 | 352.69M | 0.260 | 384.91M | nan | — | — |
| 1,000,000 | 3.225 | 310.08M | 2.815 | 355.27M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | nan | — |
| 1 | 5 | 0.343 | nan | — |
| 1 | 10 | 0.478 | nan | — |
| 10 | 1 | 0.051 | nan | — |
| 10 | 5 | 0.221 | nan | — |
| 10 | 10 | 0.451 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.248 | nan | — |
| 100 | 10 | 0.507 | nan | — |
| 1,000 | 1 | 0.057 | nan | — |
| 1,000 | 5 | 0.239 | nan | — |
| 1,000 | 10 | 0.517 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
