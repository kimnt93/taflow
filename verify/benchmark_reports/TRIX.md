# TripleExponentialRateOfChange benchmark (`TRIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 128.13M | 0.007 | 138.84M | 0.041 | 5.21× | 5.64× |
| 10,000 | 0.060 | 167.04M | 0.057 | 174.14M | 0.128 | 2.14× | 2.24× |
| 100,000 | 0.606 | 165.03M | 0.609 | 164.08M | 1.037 | 1.71× | 1.70× |
| 1,000,000 | 7.809 | 128.05M | 7.148 | 139.90M | 11.142 | 1.43× | 1.56× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.600 ms**; native kernel **0.614 ms**; TA-Lib 1.089 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.249 | 0.169 | 5.90M | 1012.028 | 5973.27× | 181.14× |
| 100,000 | 10 | 0.999 | 0.551 | 18.16M | 955.390 | 1734.54× | 55.24× |
| 100,000 | 1,000 | 9.257 | 7.814 | 127.97M | 996.438 | 127.51× | 5.27× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 115.04M | 100.74M | 1.00× | 2.38M | 3.14M | 1.00× | 85.42M |
| 2 | 222.65M | 250.75M | 2.49× | 2.58M | 2.76M | 0.88× | 89.71M |
| 4 | 269.07M | 350.59M | 3.48× | 2.72M | 2.97M | 0.95× | 90.17M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
