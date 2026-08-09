# MovingAverageConvergenceDivergence benchmark (`MACD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 157.77M | 0.005 | 209.31M | 0.053 | 8.36× | 11.09× |
| 10,000 | 0.033 | 299.18M | 0.026 | 391.23M | 0.138 | 4.14× | 5.41× |
| 100,000 | 0.298 | 336.03M | 0.248 | 403.47M | 1.016 | 3.41× | 4.10× |
| 1,000,000 | 12.872 | 77.69M | 2.564 | 390.02M | 14.973 | 1.16× | 5.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.175 | 2.18× |
| 1 | 5 | 0.309 | 0.634 | 2.05× |
| 1 | 10 | 0.505 | 1.075 | 2.13× |
| 10 | 1 | 0.050 | 0.100 | 2.00× |
| 10 | 5 | 0.238 | 0.573 | 2.41× |
| 10 | 10 | 0.458 | 1.041 | 2.27× |
| 100 | 1 | 0.052 | 0.107 | 2.04× |
| 100 | 5 | 0.228 | 0.519 | 2.28× |
| 100 | 10 | 0.489 | 1.116 | 2.28× |
| 1,000 | 1 | 0.058 | 0.123 | 2.12× |
| 1,000 | 5 | 0.287 | 0.560 | 1.95× |
| 1,000 | 10 | 0.534 | 1.247 | 2.33× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.376 | 0.278 | 3.60M | 988.093 | 3558.15× | 159.38× |
| 100,000 | 10 | 1.907 | 1.154 | 8.66M | 1018.342 | 882.21× | 38.18× |
| 100,000 | 1,000 | 87.539 | 86.440 | 11.57M | 996.254 | 11.53× | 0.62× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 104.76M | 233.35M | 1.00× | 1.86M | 1.86M | 1.00× | 81.41M |
| 5 | 160.68M | 462.28M | 1.98× | 1.42M | 1.27M | 0.69× | 79.36M |
| 10 | 193.23M | 531.66M | 2.28× | 1.26M | 1.22M | 0.65× | 79.58M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
