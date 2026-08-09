# NormalizedAverageTrueRange benchmark (`NATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.08M | 0.007 | 134.75M | 0.037 | 4.18× | 4.98× |
| 10,000 | 0.069 | 144.04M | 0.064 | 157.12M | 0.087 | 1.25× | 1.36× |
| 100,000 | 0.672 | 148.87M | 0.656 | 152.53M | 0.579 | 0.86× | 0.88× |
| 1,000,000 | 6.818 | 146.67M | 6.459 | 154.82M | 6.208 | 0.91× | 0.96× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.658 ms**; native kernel **0.618 ms**; TA-Lib 0.568 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.309 | 0.218 | 4.59M | 642.221 | 2945.45× | 146.91× |
| 100,000 | 10 | 1.904 | 0.946 | 10.58M | 560.177 | 592.41× | 32.85× |
| 100,000 | 1,000 | 10.080 | 11.157 | 89.63M | 582.883 | 52.24× | 3.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 122.54M | 129.12M | 1.00× | 2.76M | 3.42M | 1.00× | 122.55M |
| 2 | 236.58M | 251.90M | 1.95× | 2.35M | 3.00M | 0.88× | 131.63M |
| 4 | 366.98M | 461.22M | 3.57× | 2.28M | 2.73M | 0.80× | 129.10M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
