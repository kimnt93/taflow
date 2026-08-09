# RollingMode benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.199 | 5.03M | 0.200 | 4.99M | nan | — | — |
| 10,000 | 2.063 | 4.85M | 2.121 | 4.71M | nan | — | — |
| 100,000 | 20.237 | 4.94M | 20.636 | 4.85M | nan | — | — |
| 1,000,000 | 201.745 | 4.96M | 208.166 | 4.80M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | nan | — |
| 1 | 5 | 0.303 | nan | — |
| 1 | 10 | 0.489 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.237 | nan | — |
| 10 | 10 | 0.509 | nan | — |
| 100 | 1 | 0.073 | nan | — |
| 100 | 5 | 0.263 | nan | — |
| 100 | 10 | 0.510 | nan | — |
| 1,000 | 1 | 0.264 | nan | — |
| 1,000 | 5 | 0.458 | nan | — |
| 1,000 | 10 | 0.710 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
