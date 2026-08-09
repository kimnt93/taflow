# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.48M | 0.012 | 84.15M | 0.038 | 2.85× | 3.22× |
| 10,000 | 0.153 | 65.43M | 0.148 | 67.73M | 0.183 | 1.20× | 1.24× |
| 100,000 | 1.497 | 66.78M | 1.480 | 67.58M | 1.567 | 1.05× | 1.06× |
| 1,000,000 | 15.284 | 65.43M | 14.830 | 67.43M | 14.983 | 0.98× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.124 | 1.05× |
| 1 | 5 | 0.326 | 0.534 | 1.64× |
| 1 | 10 | 0.479 | 0.911 | 1.90× |
| 10 | 1 | 0.048 | 0.082 | 1.72× |
| 10 | 5 | 0.214 | 0.427 | 1.99× |
| 10 | 10 | 0.486 | 0.916 | 1.89× |
| 100 | 1 | 0.053 | 0.094 | 1.78× |
| 100 | 5 | 0.230 | 0.424 | 1.84× |
| 100 | 10 | 0.467 | 0.879 | 1.88× |
| 1,000 | 1 | 0.062 | 0.100 | 1.62× |
| 1,000 | 5 | 0.240 | 0.501 | 2.09× |
| 1,000 | 10 | 0.492 | 1.046 | 2.13× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.311 | 0.173 | 5.77M | 1483.637 | 8558.92× | 145.01× |
| 100,000 | 10 | 1.094 | 0.658 | 15.20M | 1472.801 | 2238.85× | 42.10× |
| 100,000 | 1,000 | 20.219 | 40.157 | 24.90M | 1563.340 | 38.93× | 1.20× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 61.12M | 65.05M | 1.00× | 2.75M | 3.67M | 1.00× | 63.23M |
| 5 | 211.91M | 218.40M | 3.36× | 2.13M | 2.66M | 0.72× | 58.22M |
| 10 | 299.88M | 347.28M | 5.34× | 2.05M | 2.52M | 0.69× | 58.83M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
