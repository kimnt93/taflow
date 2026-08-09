# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.65M | 0.005 | 206.92M | 0.031 | 5.29× | 6.45× |
| 10,000 | 0.031 | 319.20M | 0.028 | 351.68M | 0.042 | 1.34× | 1.48× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.006 ms**; TA-Lib 0.027 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.289 | 0.163 | 6.12M | 28.004 | 171.41× | 161.70× |
| 1,500 | 10 | 1.594 | 0.582 | 17.19M | 27.818 | 47.82× | 45.72× |
| 1,500 | 100 | 3.210 | 1.873 | 53.39M | 29.181 | 15.58× | 14.13× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.73M | 18.11M | 1.00× | 1.19M | 1.04M | 1.00× | 10.18M |
| 2 | 19.62M | 21.28M | 1.17× | 1.45M | 1.62M | 1.55× | 10.73M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
