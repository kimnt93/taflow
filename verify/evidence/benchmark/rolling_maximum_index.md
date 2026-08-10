# RollingMaximumIndex benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 123.83M | 0.007 | 143.21M | 0.038 | 4.73× | 5.47× |
| 10,000 | 0.062 | 161.79M | 0.054 | 184.07M | 0.102 | 1.65× | 1.88× |
| 100,000 | 0.554 | 180.58M | 0.521 | 192.01M | 0.708 | 1.28× | 1.36× |
| 1,000,000 | 5.680 | 176.06M | 5.433 | 184.05M | 7.021 | 1.24× | 1.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.110 | 1.24× |
| 1 | 5 | 0.331 | 0.514 | 1.56× |
| 1 | 10 | 0.503 | 0.972 | 1.93× |
| 10 | 1 | 0.048 | 0.089 | 1.85× |
| 10 | 5 | 0.228 | 0.457 | 2.00× |
| 10 | 10 | 0.529 | 0.953 | 1.80× |
| 100 | 1 | 0.046 | 0.091 | 1.98× |
| 100 | 5 | 0.231 | 0.432 | 1.87× |
| 100 | 10 | 0.510 | 1.014 | 1.99× |
| 1,000 | 1 | 0.069 | 0.096 | 1.40× |
| 1,000 | 5 | 0.229 | 0.462 | 2.02× |
| 1,000 | 10 | 0.482 | 1.050 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
