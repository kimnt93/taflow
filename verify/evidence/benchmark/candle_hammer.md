# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.017 | 58.06M | 0.010 | 99.17M | 0.040 | 2.33× | 3.98× |
| 10,000 | 0.111 | 90.24M | 0.110 | 90.96M | 0.210 | 1.89× | 1.91× |
| 100,000 | 1.269 | 78.83M | 1.254 | 79.72M | 1.503 | 1.18× | 1.20× |
| 1,000,000 | 13.000 | 76.92M | 12.925 | 77.37M | 15.241 | 1.17× | 1.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.114 | 1.39× |
| 1 | 5 | 0.273 | 0.479 | 1.76× |
| 1 | 10 | 0.524 | 0.988 | 1.89× |
| 10 | 1 | 0.062 | 0.105 | 1.69× |
| 10 | 5 | 0.275 | 0.432 | 1.57× |
| 10 | 10 | 0.528 | 0.891 | 1.69× |
| 100 | 1 | 0.062 | 0.101 | 1.63× |
| 100 | 5 | 0.289 | 0.484 | 1.68× |
| 100 | 10 | 0.582 | 0.961 | 1.65× |
| 1,000 | 1 | 0.075 | 0.125 | 1.67× |
| 1,000 | 5 | 0.288 | 0.565 | 1.96× |
| 1,000 | 10 | 0.620 | 1.110 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
