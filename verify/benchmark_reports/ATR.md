# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 132.89M | 0.006 | 165.58M | 0.037 | 4.87× | 6.07× |
| 10,000 | 0.057 | 176.91M | 0.052 | 194.05M | 0.085 | 1.51× | 1.66× |
| 100,000 | 0.511 | 195.80M | 0.487 | 205.29M | 0.564 | 1.10× | 1.16× |
| 1,000,000 | 5.630 | 177.61M | 5.296 | 188.84M | 6.058 | 1.08× | 1.14× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.510 ms**; native kernel **0.496 ms**; TA-Lib 0.569 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.319 | 0.223 | 4.49M | 581.977 | 2615.44× | 141.42× |
| 100,000 | 10 | 1.958 | 0.976 | 10.25M | 560.617 | 574.58× | 31.57× |
| 100,000 | 1,000 | 9.495 | 7.654 | 130.66M | 584.328 | 76.35× | 4.78× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 142.64M | 146.76M | 1.00× | 2.64M | 3.21M | 1.00× | 130.36M |
| 2 | 284.60M | 317.68M | 2.16× | 2.32M | 3.04M | 0.95× | 134.26M |
| 4 | 422.66M | 565.94M | 3.86× | 2.17M | 2.53M | 0.79× | 127.11M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
