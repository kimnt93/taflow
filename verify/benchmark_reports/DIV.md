# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 203.58M | 0.003 | 294.49M | 0.028 | 5.73× | 8.30× |
| 10,000 | 0.012 | 862.29M | 0.008 | 1.23G | 0.033 | 2.87× | 4.08× |
| 100,000 | 0.073 | 1.37G | 0.049 | 2.06G | 0.073 | 0.99× | 1.50× |
| 1,000,000 | 1.163 | 860.03M | 0.913 | 1.10G | 0.783 | 0.67× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.167 | 0.172 | 1.03× |
| 1 | 5 | 0.341 | 0.521 | 1.53× |
| 1 | 10 | 0.510 | 0.922 | 1.81× |
| 10 | 1 | 0.051 | 0.090 | 1.77× |
| 10 | 5 | 0.237 | 0.466 | 1.96× |
| 10 | 10 | 0.495 | 0.947 | 1.91× |
| 100 | 1 | 0.051 | 0.087 | 1.70× |
| 100 | 5 | 0.238 | 0.440 | 1.85× |
| 100 | 10 | 0.465 | 0.910 | 1.96× |
| 1,000 | 1 | 0.051 | 0.090 | 1.77× |
| 1,000 | 5 | 0.275 | 0.466 | 1.69× |
| 1,000 | 10 | 0.543 | 0.968 | 1.78× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.347 | 0.195 | 5.14M | 75.247 | 386.66× | 141.21× |
| 100,000 | 10 | 1.602 | 0.689 | 14.52M | 76.198 | 110.63× | 40.06× |
| 100,000 | 1,000 | 4.018 | 2.273 | 439.89M | 77.595 | 34.13× | 12.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 560.15M | 709.22M | 1.00× | 2.53M | 3.24M | 1.00× | 559.22M |
| 5 | 787.04M | 1.48G | 2.09× | 2.18M | 2.88M | 0.89× | 619.13M |
| 10 | 672.41M | 1.41G | 1.98× | 2.10M | 2.51M | 0.78× | 610.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
