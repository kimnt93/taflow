# ExponentiallyWeightedCorrelation benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.38M | 0.009 | 117.02M | nan | — | — |
| 10,000 | 0.079 | 126.41M | 0.058 | 173.41M | nan | — | — |
| 100,000 | 0.572 | 174.73M | 0.529 | 189.16M | nan | — | — |
| 1,000,000 | 5.739 | 174.24M | 5.514 | 181.34M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | nan | — |
| 1 | 5 | 0.252 | nan | — |
| 1 | 10 | 0.459 | nan | — |
| 10 | 1 | 0.050 | nan | — |
| 10 | 5 | 0.222 | nan | — |
| 10 | 10 | 0.480 | nan | — |
| 100 | 1 | 0.049 | nan | — |
| 100 | 5 | 0.225 | nan | — |
| 100 | 10 | 0.458 | nan | — |
| 1,000 | 1 | 0.052 | nan | — |
| 1,000 | 5 | 0.233 | nan | — |
| 1,000 | 10 | 0.498 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
