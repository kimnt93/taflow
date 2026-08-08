# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 368.66M | 0.002 | 596.14M | 0.027 | 9.92× | 16.05× |
| 10,000 | 0.014 | 723.59M | 0.012 | 838.95M | 0.041 | 2.93× | 3.40× |
| 100,000 | 0.152 | 659.62M | 0.125 | 796.87M | 0.162 | 1.07× | 1.29× |
| 1,000,000 | 2.837 | 352.48M | 2.266 | 441.31M | 1.452 | 0.51× | 0.64× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.151 ms**; native kernel **0.127 ms**; TA-Lib 0.157 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.218 | 0.149 | 6.71M | 153.173 | 1027.04× | 164.57× |
| 100,000 | 10 | 0.912 | 0.609 | 16.42M | 160.272 | 263.17× | 40.29× |
| 100,000 | 1,000 | 4.008 | 2.925 | 341.88M | 156.266 | 53.42× | 8.87× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 267.77M | 358.21M | 1.00× | 3.30M | 3.27M | 1.00× | 408.40M |
| 2 | 475.09M | 754.45M | 2.11× | 3.13M | 3.89M | 1.19× | 413.47M |
| 4 | 430.90M | 863.71M | 2.41× | 2.99M | 3.37M | 1.03× | 397.88M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
