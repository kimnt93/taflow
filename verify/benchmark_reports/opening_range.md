# OpeningRange benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.96M | 0.011 | 90.08M | nan | — | — |
| 10,000 | 0.079 | 125.88M | 0.072 | 138.26M | nan | — | — |
| 100,000 | 0.709 | 141.03M | 0.627 | 159.50M | nan | — | — |
| 1,000,000 | 8.312 | 120.30M | 6.936 | 144.18M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | nan | — |
| 1 | 5 | 0.411 | nan | — |
| 1 | 10 | 0.486 | nan | — |
| 10 | 1 | 0.049 | nan | — |
| 10 | 5 | 0.221 | nan | — |
| 10 | 10 | 0.452 | nan | — |
| 100 | 1 | 0.050 | nan | — |
| 100 | 5 | 0.217 | nan | — |
| 100 | 10 | 0.577 | nan | — |
| 1,000 | 1 | 0.071 | nan | — |
| 1,000 | 5 | 0.309 | nan | — |
| 1,000 | 10 | 0.577 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
