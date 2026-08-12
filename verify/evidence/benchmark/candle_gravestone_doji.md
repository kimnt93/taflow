# CandleGravestoneDoji benchmark (`CDLGRAVESTONEDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 75.85M | 0.010 | 96.83M | 0.036 | 2.72× | 3.47× |
| 10,000 | 0.072 | 139.01M | 0.067 | 148.31M | 0.100 | 1.39× | 1.48× |
| 100,000 | 0.768 | 130.25M | 0.754 | 132.60M | 0.751 | 0.98× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.154 | 0.145 | 0.94× |
| 1 | 5 | 0.338 | 0.483 | 1.43× |
| 1 | 10 | 0.545 | 0.959 | 1.76× |
| 10 | 1 | 0.054 | 0.088 | 1.62× |
| 10 | 5 | 0.291 | 0.464 | 1.59× |
| 10 | 10 | 0.630 | 0.942 | 1.49× |
| 100 | 1 | 0.058 | 0.095 | 1.65× |
| 100 | 5 | 0.267 | 0.522 | 1.95× |
| 100 | 10 | 0.576 | 0.968 | 1.68× |
| 1,000 | 1 | 0.071 | 0.096 | 1.35× |
| 1,000 | 5 | 0.283 | 0.485 | 1.71× |
| 1,000 | 10 | 0.542 | 1.058 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
