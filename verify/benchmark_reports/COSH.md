# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.07M | 0.007 | 137.75M | 0.032 | 3.91× | 4.44× |
| 10,000 | 0.069 | 145.42M | 0.069 | 144.12M | 0.094 | 1.37× | 1.36× |
| 100,000 | 0.669 | 149.42M | 0.634 | 157.74M | 0.601 | 0.90× | 0.95× |
| 1,000,000 | 7.352 | 136.01M | 6.932 | 144.26M | 5.778 | 0.79× | 0.83× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.653 ms**; native kernel **0.627 ms**; TA-Lib 0.603 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.260 | 0.169 | 5.91M | 598.432 | 3538.77× | 154.01× |
| 100,000 | 10 | 0.965 | 0.656 | 15.24M | 594.923 | 906.57× | 39.33× |
| 100,000 | 1,000 | 9.270 | 7.592 | 131.71M | 610.613 | 80.42× | 4.12× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 124.34M | 116.66M | 1.00× | 2.77M | 3.17M | 1.00× | 147.22M |
| 2 | 230.36M | 228.03M | 1.95× | 2.92M | 3.61M | 1.14× | 144.86M |
| 4 | 294.20M | 379.90M | 3.26× | 2.84M | 2.91M | 0.92× | 140.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
