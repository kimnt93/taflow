# ChaikinVolatility benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.29M | 0.009 | 113.34M | nan | — | — |
| 10,000 | 0.058 | 172.06M | 0.056 | 179.29M | nan | — | — |
| 100,000 | 0.586 | 170.67M | 0.511 | 195.80M | nan | — | — |
| 1,000,000 | 6.202 | 161.24M | 5.382 | 185.79M | nan | — | — |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | nan | — |
| 1 | 5 | 0.340 | nan | — |
| 1 | 10 | 0.468 | nan | — |
| 10 | 1 | 0.053 | nan | — |
| 10 | 5 | 0.231 | nan | — |
| 10 | 10 | 0.465 | nan | — |
| 100 | 1 | 0.047 | nan | — |
| 100 | 5 | 0.237 | nan | — |
| 100 | 10 | 0.474 | nan | — |
| 1,000 | 1 | 0.053 | nan | — |
| 1,000 | 5 | 0.237 | nan | — |
| 1,000 | 10 | 0.488 | nan | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
