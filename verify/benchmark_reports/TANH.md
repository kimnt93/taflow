# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.83M | 0.005 | 208.07M | 0.028 | 4.75× | 5.93× |
| 10,000 | 0.032 | 312.04M | 0.028 | 358.60M | 0.053 | 1.66× | 1.90× |
| 100,000 | 0.272 | 367.22M | 0.249 | 401.43M | 0.275 | 1.01× | 1.10× |
| 1,000,000 | 2.913 | 343.27M | 2.634 | 379.71M | 2.610 | 0.90× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.119 | 1.07× |
| 1 | 5 | 0.279 | 0.473 | 1.69× |
| 1 | 10 | 0.450 | 0.891 | 1.98× |
| 10 | 1 | 0.048 | 0.084 | 1.75× |
| 10 | 5 | 0.215 | 0.412 | 1.92× |
| 10 | 10 | 0.483 | 0.887 | 1.84× |
| 100 | 1 | 0.049 | 0.085 | 1.75× |
| 100 | 5 | 0.239 | 0.405 | 1.69× |
| 100 | 10 | 0.445 | 0.853 | 1.92× |
| 1,000 | 1 | 0.052 | 0.090 | 1.73× |
| 1,000 | 5 | 0.235 | 0.429 | 1.83× |
| 1,000 | 10 | 0.473 | 0.893 | 1.89× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.290 | 0.159 | 6.29M | 279.328 | 1756.32× | 153.75× |
| 100,000 | 10 | 0.910 | 0.470 | 21.28M | 282.504 | 601.13× | 53.87× |
| 100,000 | 1,000 | 7.490 | 3.953 | 252.95M | 286.921 | 72.58× | 7.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 240.90M | 242.60M | 1.00× | 2.61M | 4.19M | 1.00× | 218.67M |
| 5 | 638.28M | 1.02G | 4.20× | 2.42M | 3.24M | 0.77× | 254.13M |
| 10 | 672.54M | 1.21G | 4.99× | 2.27M | 3.13M | 0.75× | 270.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
