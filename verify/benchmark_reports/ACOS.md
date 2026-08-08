# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.07M | 0.008 | 122.29M | 0.034 | 3.90× | 4.21× |
| 10,000 | 0.077 | 130.37M | 0.078 | 127.71M | 0.103 | 1.34× | 1.31× |
| 100,000 | 0.806 | 124.13M | 0.751 | 133.16M | 0.701 | 0.87× | 0.93× |
| 1,000,000 | 8.762 | 114.14M | 8.850 | 113.00M | 6.706 | 0.77× | 0.76× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.781 ms**; native kernel **0.759 ms**; TA-Lib 0.685 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.227 | 0.169 | 5.91M | 706.220 | 4172.52× | 159.94× |
| 100,000 | 10 | 1.027 | 0.742 | 13.49M | 689.915 | 930.36× | 37.58× |
| 100,000 | 1,000 | 17.824 | 17.094 | 58.50M | 693.668 | 40.58× | 1.90× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 104.83M | 120.05M | 1.00× | 3.10M | 3.16M | 1.00× | 127.35M |
| 2 | 204.70M | 219.98M | 1.83× | 3.25M | 3.04M | 0.96× | 117.10M |
| 4 | 270.35M | 307.81M | 2.56× | 2.79M | 3.13M | 0.99× | 120.79M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
