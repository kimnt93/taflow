# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.80M | 0.011 | 95.14M | 0.034 | 2.90× | 3.21× |
| 10,000 | 0.087 | 114.51M | 0.084 | 119.07M | 0.105 | 1.21× | 1.25× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.016 ms**; native kernel **0.015 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.298 | 0.172 | 5.81M | 40.252 | 233.76× | 153.37× |
| 1,500 | 10 | 1.236 | 0.689 | 14.51M | 39.141 | 56.79× | 36.83× |
| 1,500 | 100 | 3.744 | 2.630 | 38.02M | 42.179 | 16.04× | 11.51× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.01M | 14.93M | 1.00× | 1.13M | 1.41M | 1.00× | 9.63M |
| 2 | 16.88M | 23.39M | 1.57× | 1.45M | 1.63M | 1.15× | 10.12M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
