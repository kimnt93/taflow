# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.45M | 0.009 | 108.63M | 0.038 | 3.64× | 4.10× |
| 10,000 | 0.057 | 175.16M | 0.054 | 186.31M | 0.086 | 1.51× | 1.60× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.013 ms**; native kernel **0.012 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.416 | 0.234 | 4.27M | 40.699 | 173.91× | 132.11× |
| 1,500 | 10 | 2.094 | 1.022 | 9.78M | 39.604 | 38.75× | 31.07× |
| 1,500 | 100 | 4.236 | 2.514 | 39.77M | 40.550 | 16.13× | 13.18× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.65M | 14.61M | 1.00× | 1.22M | 1.40M | 1.00× | 9.33M |
| 2 | 16.70M | 21.95M | 1.50× | 1.20M | 1.25M | 0.90× | 8.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
