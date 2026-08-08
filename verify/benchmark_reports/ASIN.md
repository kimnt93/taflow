# MathAsin benchmark (`ASIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.85M | 0.008 | 133.20M | 0.033 | 3.77× | 4.45× |
| 10,000 | 0.075 | 133.79M | 0.072 | 138.62M | 0.097 | 1.30× | 1.34× |
| 100,000 | 0.755 | 132.38M | 0.791 | 126.43M | 0.737 | 0.98× | 0.93× |
| 1,000,000 | 8.665 | 115.41M | 8.105 | 123.38M | 6.948 | 0.80× | 0.86× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.796 ms**; native kernel **0.769 ms**; TA-Lib 0.691 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.223 | 0.157 | 6.36M | 649.770 | 4135.07× | 163.54× |
| 100,000 | 10 | 1.038 | 0.646 | 15.48M | 725.741 | 1123.64× | 40.51× |
| 100,000 | 1,000 | 10.283 | 8.848 | 113.02M | 856.769 | 96.83× | 3.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 109.33M | 124.50M | 1.00× | 2.74M | 3.37M | 1.00× | 127.08M |
| 2 | 214.39M | 217.48M | 1.75× | 2.88M | 3.38M | 1.00× | 124.32M |
| 4 | 288.32M | 377.87M | 3.04× | 2.71M | 3.02M | 0.90× | 124.16M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
