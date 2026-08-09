# NegativeVolumeIndex benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.16M | 0.008 | 120.58M | nan | — | — |
| 10,000 | 0.061 | 163.56M | 0.060 | 167.16M | nan | — | — |
| 100,000 | 0.595 | 167.93M | 0.593 | 168.61M | nan | — | — |
| 1,000,000 | 6.341 | 157.71M | 5.882 | 170.01M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | nan | — |
| 1 | 5 | 0.295 | nan | — |
| 1 | 10 | 0.482 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.224 | nan | — |
| 10 | 10 | 0.482 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.230 | nan | — |
| 100 | 10 | 0.489 | nan | — |
| 1,000 | 1 | 0.055 | nan | — |
| 1,000 | 5 | 0.249 | nan | — |
| 1,000 | 10 | 0.505 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
