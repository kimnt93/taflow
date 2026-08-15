# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.43M | 0.004 | 281.36M | 0.031 | 4.92× | 8.79× |
| 10,000 | 0.051 | 197.32M | 0.048 | 210.24M | 0.087 | 1.71× | 1.82× |
| 100,000 | 0.558 | 179.33M | 0.544 | 183.94M | 0.625 | 1.12× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.063 | 0.124 | 1.97× |
| 1 | 5 | 0.201 | 0.473 | 2.35× |
| 1 | 10 | 0.385 | 0.871 | 2.26× |
| 10 | 1 | 0.044 | 0.095 | 2.18× |
| 10 | 5 | 0.175 | 0.412 | 2.35× |
| 10 | 10 | 0.372 | 0.861 | 2.31× |
| 100 | 1 | 0.039 | 0.083 | 2.13× |
| 100 | 5 | 0.182 | 0.404 | 2.21× |
| 100 | 10 | 0.393 | 0.878 | 2.24× |
| 1,000 | 1 | 0.046 | 0.096 | 2.11× |
| 1,000 | 5 | 0.191 | 0.474 | 2.48× |
| 1,000 | 10 | 0.428 | 1.412 | 3.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
