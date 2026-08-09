# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 110.62M | 0.008 | 128.89M | 0.033 | 3.67× | 4.27× |
| 10,000 | 0.079 | 127.07M | 0.084 | 118.88M | 0.094 | 1.19× | 1.12× |
| 100,000 | 0.763 | 131.09M | 0.733 | 136.39M | 0.687 | 0.90× | 0.94× |
| 1,000,000 | 8.302 | 120.45M | 7.886 | 126.81M | 6.676 | 0.80× | 0.85× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.763 ms**; native kernel **0.738 ms**; TA-Lib 0.686 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.238 | 0.159 | 6.28M | 681.784 | 4282.75× | 158.31× |
| 100,000 | 10 | 1.087 | 0.599 | 16.69M | 688.758 | 1149.65× | 43.06× |
| 100,000 | 1,000 | 10.592 | 9.080 | 110.13M | 696.897 | 76.75× | 3.58× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 107.81M | 118.41M | 1.00× | 2.94M | 3.56M | 1.00× | 126.43M |
| 2 | 193.11M | 220.33M | 1.86× | 2.93M | 3.25M | 0.91× | 120.27M |
| 4 | 265.40M | 333.20M | 2.81× | 2.77M | 2.94M | 0.83× | 119.53M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
