# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 79.08M | 0.011 | 89.23M | 0.047 | 3.72× | 4.20× |
| 10,000 | 0.109 | 91.48M | 0.103 | 96.65M | 0.162 | 1.48× | 1.56× |
| 100,000 | 1.107 | 90.30M | 1.056 | 94.71M | 1.332 | 1.20× | 1.26× |
| 1,000,000 | 11.876 | 84.20M | 10.536 | 94.91M | 12.152 | 1.02× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.156 | 1.73× |
| 1 | 5 | 0.273 | 0.530 | 1.94× |
| 1 | 10 | 0.486 | 1.067 | 2.20× |
| 10 | 1 | 0.056 | 0.111 | 1.99× |
| 10 | 5 | 0.238 | 0.485 | 2.04× |
| 10 | 10 | 0.489 | 1.090 | 2.23× |
| 100 | 1 | 0.065 | 0.105 | 1.60× |
| 100 | 5 | 0.236 | 0.500 | 2.12× |
| 100 | 10 | 0.534 | 1.056 | 1.98× |
| 1,000 | 1 | 0.061 | 0.124 | 2.02× |
| 1,000 | 5 | 0.290 | 0.615 | 2.12× |
| 1,000 | 10 | 0.512 | 1.145 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
