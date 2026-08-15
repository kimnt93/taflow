# MinusDirectionalIndicator benchmark (`MINUS_DI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 129.10M | 0.007 | 152.40M | 0.038 | 4.85× | 5.73× |
| 10,000 | 0.061 | 165.24M | 0.059 | 168.16M | 0.094 | 1.55× | 1.58× |
| 100,000 | 0.593 | 168.63M | 0.565 | 177.12M | 0.639 | 1.08× | 1.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.154 | 1.56× |
| 1 | 5 | 0.342 | 0.511 | 1.49× |
| 1 | 10 | 0.385 | 0.943 | 2.45× |
| 10 | 1 | 0.040 | 0.092 | 2.30× |
| 10 | 5 | 0.193 | 0.476 | 2.47× |
| 10 | 10 | 0.389 | 0.936 | 2.40× |
| 100 | 1 | 0.044 | 0.091 | 2.07× |
| 100 | 5 | 0.192 | 0.453 | 2.36× |
| 100 | 10 | 0.396 | 0.936 | 2.36× |
| 1,000 | 1 | 0.047 | 0.097 | 2.05× |
| 1,000 | 5 | 0.186 | 0.493 | 2.65× |
| 1,000 | 10 | 0.415 | 1.015 | 2.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
