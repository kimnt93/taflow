# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.95M | 0.008 | 117.99M | 0.039 | 4.22× | 4.61× |
| 10,000 | 0.077 | 130.54M | 0.071 | 140.99M | 0.084 | 1.10× | 1.18× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.353 | 0.273 | 3.66M | 33.539 | 122.66× | 107.36× |
| 1,500 | 10 | 2.613 | 1.313 | 7.62M | 33.372 | 25.42× | 21.81× |
| 1,500 | 100 | 5.500 | 3.087 | 32.39M | 31.709 | 10.27× | 9.69× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.99M | 13.63M | 1.00× | 1.13M | 1.04M | 1.00× | 8.92M |
| 2 | 17.01M | 17.48M | 1.28× | 1.25M | 1.36M | 1.31× | 8.42M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
