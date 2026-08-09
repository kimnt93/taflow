# Drawdown benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.78M | 0.006 | 161.35M | nan | — | — |
| 10,000 | 0.042 | 237.82M | 0.040 | 252.86M | nan | — | — |
| 100,000 | 0.394 | 254.12M | 0.376 | 266.10M | nan | — | — |
| 1,000,000 | 4.261 | 234.71M | 3.795 | 263.51M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | nan | — |
| 1 | 5 | 0.358 | nan | — |
| 1 | 10 | 0.485 | nan | — |
| 10 | 1 | 0.046 | nan | — |
| 10 | 5 | 0.229 | nan | — |
| 10 | 10 | 0.469 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.213 | nan | — |
| 100 | 10 | 0.463 | nan | — |
| 1,000 | 1 | 0.051 | nan | — |
| 1,000 | 5 | 0.253 | nan | — |
| 1,000 | 10 | 0.494 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
