# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 180.41M | 0.005 | 214.11M | 0.038 | 6.89× | 8.18× |
| 10,000 | 0.026 | 389.77M | 0.023 | 437.64M | 0.055 | 2.16× | 2.43× |
| 100,000 | 0.224 | 446.10M | 0.198 | 504.21M | 0.219 | 0.98× | 1.11× |
| 1,000,000 | 2.542 | 393.47M | 2.080 | 480.81M | 1.973 | 0.78× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.171 | 1.89× |
| 1 | 5 | 0.282 | 0.515 | 1.82× |
| 1 | 10 | 0.483 | 1.003 | 2.08× |
| 10 | 1 | 0.052 | 0.107 | 2.04× |
| 10 | 5 | 0.234 | 0.470 | 2.01× |
| 10 | 10 | 0.472 | 0.998 | 2.11× |
| 100 | 1 | 0.054 | 0.100 | 1.87× |
| 100 | 5 | 0.252 | 0.483 | 1.92× |
| 100 | 10 | 0.486 | 0.990 | 2.04× |
| 1,000 | 1 | 0.058 | 0.100 | 1.72× |
| 1,000 | 5 | 0.240 | 0.490 | 2.04× |
| 1,000 | 10 | 0.514 | 1.015 | 1.97× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.234 | 0.159 | 6.30M | 218.472 | 1376.00× | 215.77× |
| 100,000 | 10 | 0.991 | 0.565 | 17.69M | 223.189 | 394.84× | 58.87× |
| 100,000 | 1,000 | 4.652 | 3.482 | 287.17M | 226.237 | 64.97× | 10.19× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 218.48M | 306.49M | 1.00× | 3.37M | 4.00M | 1.00× | 321.02M |
| 5 | 480.27M | 1.04G | 3.38× | 3.12M | 3.43M | 0.86× | 311.48M |
| 10 | 533.48M | 1.23G | 4.01× | 2.99M | 3.16M | 0.79× | 319.33M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
