# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 115.34M | 0.008 | 123.53M | 0.043 | 4.92× | 5.27× |
| 10,000 | 0.079 | 126.97M | 0.072 | 138.52M | 0.094 | 1.19× | 1.30× |
| 100,000 | 0.756 | 132.29M | 0.694 | 144.08M | 0.648 | 0.86× | 0.93× |
| 1,000,000 | 7.771 | 128.69M | 7.343 | 136.19M | 6.136 | 0.79× | 0.84× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.710 ms**; native kernel **0.683 ms**; TA-Lib 0.628 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.244 | 0.162 | 6.18M | 620.188 | 3835.31× | 161.11× |
| 100,000 | 10 | 1.012 | 0.612 | 16.35M | 635.733 | 1039.49× | 41.13× |
| 100,000 | 1,000 | 9.897 | 8.695 | 115.01M | 661.161 | 76.04× | 3.60× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 109.56M | 130.25M | 1.00× | 2.79M | 3.08M | 1.00× | 136.90M |
| 2 | 221.46M | 255.60M | 1.96× | 3.44M | 3.50M | 1.14× | 130.36M |
| 4 | 282.64M | 362.52M | 2.78× | 2.69M | 2.78M | 0.90× | 135.90M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
