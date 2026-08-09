# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.36M | 0.004 | 279.69M | 0.035 | 6.52× | 9.69× |
| 10,000 | 0.050 | 198.48M | 0.047 | 212.59M | 0.095 | 1.88× | 2.01× |
| 100,000 | 0.554 | 180.64M | 0.548 | 182.36M | 0.695 | 1.25× | 1.27× |
| 1,000,000 | 5.886 | 169.91M | 5.994 | 166.83M | 7.069 | 1.20× | 1.18× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.555 ms**; native kernel **0.544 ms**; TA-Lib 0.692 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.313 | 0.251 | 3.99M | 695.764 | 2773.46× | 109.62× |
| 100,000 | 10 | 2.395 | 1.228 | 8.14M | 689.742 | 561.59× | 22.18× |
| 100,000 | 1,000 | 22.342 | 19.855 | 50.37M | 700.966 | 35.30× | 1.64× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 132.98M | 158.45M | 1.00× | 2.29M | 2.30M | 1.00× | 124.82M |
| 2 | 280.52M | 304.65M | 1.92× | 2.66M | 2.81M | 1.22× | 121.23M |
| 4 | 483.96M | 485.43M | 3.06× | 2.48M | 2.76M | 1.20× | 118.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
