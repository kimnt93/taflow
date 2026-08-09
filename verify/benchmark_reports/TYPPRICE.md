# TypicalPrice benchmark (`TYPPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.23M | 0.005 | 199.72M | 0.029 | 4.32× | 5.78× |
| 10,000 | 0.023 | 436.73M | 0.018 | 542.43M | 0.037 | 1.60× | 1.99× |
| 100,000 | 0.163 | 614.34M | 0.141 | 710.03M | 0.081 | 0.50× | 0.58× |
| 1,000,000 | 2.642 | 378.53M | 1.942 | 514.93M | 1.248 | 0.47× | 0.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.189 | 1.87× |
| 1 | 5 | 0.276 | 0.524 | 1.90× |
| 1 | 10 | 0.472 | 0.902 | 1.91× |
| 10 | 1 | 0.052 | 0.094 | 1.81× |
| 10 | 5 | 0.221 | 0.426 | 1.93× |
| 10 | 10 | 0.500 | 0.965 | 1.93× |
| 100 | 1 | 0.054 | 0.089 | 1.64× |
| 100 | 5 | 0.233 | 0.438 | 1.88× |
| 100 | 10 | 0.515 | 0.916 | 1.78× |
| 1,000 | 1 | 0.056 | 0.108 | 1.91× |
| 1,000 | 5 | 0.261 | 0.451 | 1.72× |
| 1,000 | 10 | 0.502 | 0.930 | 1.85× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.360 | 0.238 | 4.21M | 84.810 | 357.08× | 133.73× |
| 100,000 | 10 | 3.668 | 1.625 | 6.15M | 78.981 | 48.59× | 16.91× |
| 100,000 | 1,000 | 5.343 | 3.224 | 310.17M | 92.618 | 28.73× | 8.92× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 321.14M | 422.94M | 1.00× | 2.30M | 3.00M | 1.00× | 613.25M |
| 5 | 679.56M | 1.32G | 3.12× | 2.22M | 2.50M | 0.83× | 529.54M |
| 10 | 623.97M | 1.16G | 2.75× | 2.06M | 2.20M | 0.73× | 507.62M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
