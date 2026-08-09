# Crossunder benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.16M | 0.006 | 172.94M | nan | — | — |
| 10,000 | 0.033 | 299.94M | 0.032 | 317.16M | nan | — | — |
| 100,000 | 0.294 | 339.74M | 0.276 | 362.77M | nan | — | — |
| 1,000,000 | 3.407 | 293.54M | 2.871 | 348.35M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.141 | nan | — |
| 1 | 5 | 0.499 | nan | — |
| 1 | 10 | 0.485 | nan | — |
| 10 | 1 | 0.047 | nan | — |
| 10 | 5 | 0.220 | nan | — |
| 10 | 10 | 0.462 | nan | — |
| 100 | 1 | 0.048 | nan | — |
| 100 | 5 | 0.225 | nan | — |
| 100 | 10 | 0.500 | nan | — |
| 1,000 | 1 | 0.052 | nan | — |
| 1,000 | 5 | 0.232 | nan | — |
| 1,000 | 10 | 0.458 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
