# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.24M | 0.007 | 148.91M | 0.030 | 3.59× | 4.52× |
| 10,000 | 0.057 | 176.32M | 0.054 | 184.12M | 0.074 | 1.31× | 1.36× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.008 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.357 | 0.332 | 3.01M | 31.186 | 93.88× | 84.68× |
| 1,500 | 10 | 2.589 | 1.243 | 8.05M | 31.762 | 25.55× | 28.14× |
| 1,500 | 100 | 4.996 | 2.617 | 38.22M | 34.035 | 13.01× | 11.81× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.98M | 14.53M | 1.00× | 1.22M | 1.06M | 1.00× | 8.78M |
| 2 | 16.75M | 18.61M | 1.28× | 1.26M | 1.38M | 1.31× | 9.67M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
