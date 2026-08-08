# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.11M | 0.004 | 233.05M | 0.030 | 4.93× | 7.00× |
| 10,000 | 0.069 | 145.60M | 0.064 | 156.12M | 0.086 | 1.26× | 1.35× |
| 100,000 | 0.797 | 125.44M | 0.802 | 124.64M | 0.637 | 0.80× | 0.79× |
| 1,000,000 | 8.905 | 112.30M | 8.367 | 119.52M | 6.702 | 0.75× | 0.80× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.811 ms**; native kernel **0.822 ms**; TA-Lib 0.632 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.417 | 0.302 | 3.31M | 635.477 | 2104.87× | 87.51× |
| 100,000 | 10 | 2.536 | 1.243 | 8.05M | 622.026 | 500.54× | 22.09× |
| 100,000 | 1,000 | 14.435 | 11.620 | 86.06M | 652.518 | 56.15× | 2.54× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 100.95M | 97.38M | 1.00× | 2.56M | 2.46M | 1.00× | 115.43M |
| 2 | 198.28M | 210.29M | 2.16× | 2.34M | 2.66M | 1.08× | 128.17M |
| 4 | 334.55M | 364.84M | 3.75× | 2.17M | 2.48M | 1.01× | 125.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
