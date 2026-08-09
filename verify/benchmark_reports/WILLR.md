# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.43M | 0.009 | 108.29M | 0.034 | 3.15× | 3.73× |
| 10,000 | 0.090 | 110.79M | 0.086 | 116.90M | 0.108 | 1.20× | 1.26× |
| 100,000 | 0.834 | 119.90M | 0.795 | 125.79M | 0.790 | 0.95× | 0.99× |
| 1,000,000 | 9.538 | 104.84M | 9.029 | 110.75M | 7.810 | 0.82× | 0.86× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.108 | 0.98× |
| 1 | 5 | 0.319 | 0.511 | 1.60× |
| 1 | 10 | 0.471 | 0.954 | 2.02× |
| 10 | 1 | 0.050 | 0.093 | 1.85× |
| 10 | 5 | 0.223 | 0.439 | 1.96× |
| 10 | 10 | 0.495 | 0.917 | 1.85× |
| 100 | 1 | 0.055 | 0.092 | 1.69× |
| 100 | 5 | 0.230 | 0.446 | 1.94× |
| 100 | 10 | 0.525 | 0.932 | 1.78× |
| 1,000 | 1 | 0.059 | 0.099 | 1.67× |
| 1,000 | 5 | 0.238 | 0.479 | 2.01× |
| 1,000 | 10 | 0.489 | 1.025 | 2.09× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.386 | 0.262 | 3.81M | 812.941 | 3097.74× | 117.56× |
| 100,000 | 10 | 2.165 | 1.192 | 8.39M | 805.290 | 675.47× | 24.79× |
| 100,000 | 1,000 | 29.199 | 26.759 | 37.37M | 828.845 | 30.97× | 1.30× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 92.69M | 95.92M | 1.00× | 2.26M | 2.33M | 1.00× | 95.42M |
| 5 | 272.89M | 356.41M | 3.72× | 1.98M | 2.20M | 0.95× | 101.78M |
| 10 | 305.49M | 375.20M | 3.91× | 1.87M | 2.27M | 0.97× | 102.56M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
