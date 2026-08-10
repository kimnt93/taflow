# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.68M | 0.018 | 54.51M | 0.038 | 1.80× | 2.06× |
| 10,000 | 0.196 | 50.92M | 0.186 | 53.67M | 0.177 | 0.90× | 0.95× |
| 100,000 | 1.759 | 56.84M | 1.703 | 58.71M | 1.793 | 1.02× | 1.05× |
| 1,000,000 | 17.396 | 57.49M | 18.617 | 53.72M | 14.702 | 0.85× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.152 | 1.22× |
| 1 | 5 | 0.334 | 0.502 | 1.50× |
| 1 | 10 | 0.588 | 0.938 | 1.60× |
| 10 | 1 | 0.056 | 0.092 | 1.64× |
| 10 | 5 | 0.264 | 0.465 | 1.76× |
| 10 | 10 | 0.588 | 0.931 | 1.58× |
| 100 | 1 | 0.054 | 0.093 | 1.73× |
| 100 | 5 | 0.263 | 0.459 | 1.74× |
| 100 | 10 | 0.665 | 0.987 | 1.48× |
| 1,000 | 1 | 0.072 | 0.119 | 1.65× |
| 1,000 | 5 | 0.262 | 0.534 | 2.04× |
| 1,000 | 10 | 0.587 | 1.065 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
