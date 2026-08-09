# MathRadians benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 289.29M | 0.002 | 422.03M | 0.003 | 0.83× | 1.21× |
| 10,000 | 0.008 | 1.29G | 0.005 | 2.04G | 0.014 | 1.76× | 2.78× |
| 100,000 | 0.054 | 1.85G | 0.030 | 3.29G | 0.122 | 2.25× | 4.01× |
| 1,000,000 | 1.271 | 786.70M | 0.592 | 1.69G | 1.429 | 1.12× | 2.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.094 | 0.79× |
| 1 | 5 | 0.282 | 0.192 | 0.68× |
| 1 | 10 | 0.475 | 0.390 | 0.82× |
| 10 | 1 | 0.044 | 0.039 | 0.88× |
| 10 | 5 | 0.216 | 0.188 | 0.87× |
| 10 | 10 | 0.479 | 0.404 | 0.84× |
| 100 | 1 | 0.048 | 0.060 | 1.24× |
| 100 | 5 | 0.232 | 0.186 | 0.80× |
| 100 | 10 | 0.474 | 0.393 | 0.83× |
| 1,000 | 1 | 0.049 | 0.044 | 0.90× |
| 1,000 | 5 | 0.211 | 0.179 | 0.85× |
| 1,000 | 10 | 0.491 | 0.435 | 0.89× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.291 | 0.158 | 6.35M | nan | — | — |
| 100,000 | 10 | 1.003 | 0.507 | 19.71M | nan | — | — |
| 100,000 | 1,000 | 2.785 | 1.884 | 530.71M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 395.48M | 1.16G | 1.00× | 2.93M | 4.08M | 1.00× | — |
| 5 | 550.63M | 1.91G | 1.64× | 2.70M | 3.35M | 0.82× | — |
| 10 | 536.16M | 1.48G | 1.28× | 2.42M | 3.28M | 0.81× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
