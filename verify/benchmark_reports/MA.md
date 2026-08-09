# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.00M | 0.005 | 196.62M | 0.042 | 6.85× | 8.22× |
| 10,000 | 0.027 | 371.35M | 0.023 | 429.43M | 0.058 | 2.14× | 2.48× |
| 100,000 | 0.233 | 428.84M | 0.202 | 494.91M | 0.230 | 0.99× | 1.14× |
| 1,000,000 | 2.687 | 372.17M | 2.143 | 466.69M | 2.062 | 0.77× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.163 | 1.20× |
| 1 | 5 | 0.295 | 0.546 | 1.85× |
| 1 | 10 | 0.461 | 0.969 | 2.10× |
| 10 | 1 | 0.053 | 0.108 | 2.03× |
| 10 | 5 | 0.250 | 0.512 | 2.05× |
| 10 | 10 | 0.492 | 1.022 | 2.08× |
| 100 | 1 | 0.062 | 0.105 | 1.70× |
| 100 | 5 | 0.262 | 0.523 | 2.00× |
| 100 | 10 | 0.500 | 1.007 | 2.01× |
| 1,000 | 1 | 0.065 | 0.107 | 1.64× |
| 1,000 | 5 | 0.253 | 0.480 | 1.90× |
| 1,000 | 10 | 0.509 | 1.077 | 2.12× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | Reference full µs | vs full | vs bounded tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.283 | 0.193 | 5.19M | 239.865 | 1244.32× | 198.71× |
| 100,000 | 10 | 1.555 | 0.903 | 11.07M | 233.195 | 258.13× | 38.21× |
| 100,000 | 1,000 | 32.056 | 26.328 | 37.98M | 240.375 | 9.13× | 1.60× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | Reference vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 306.22M | 307.68M | 1.00× | 3.09M | 3.39M | 1.00× | 303.79M |
| 5 | 639.50M | 1.06G | 3.44× | 2.97M | 3.20M | 0.94× | 310.74M |
| 10 | 605.22M | 1.14G | 3.72× | 2.65M | 3.22M | 0.95× | 306.42M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
