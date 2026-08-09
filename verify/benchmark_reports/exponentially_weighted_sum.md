# ExponentiallyWeightedSum benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 126.99M | 0.006 | 166.61M | nan | — | — |
| 10,000 | 0.037 | 269.25M | 0.039 | 257.74M | nan | — | — |
| 100,000 | 0.404 | 247.39M | 0.325 | 307.90M | nan | — | — |
| 1,000,000 | 3.936 | 254.08M | 3.576 | 279.67M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | nan | — |
| 1 | 5 | 0.292 | nan | — |
| 1 | 10 | 0.583 | nan | — |
| 10 | 1 | 0.054 | nan | — |
| 10 | 5 | 0.257 | nan | — |
| 10 | 10 | 0.487 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.284 | nan | — |
| 100 | 10 | 0.628 | nan | — |
| 1,000 | 1 | 0.057 | nan | — |
| 1,000 | 5 | 0.305 | nan | — |
| 1,000 | 10 | 0.652 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
