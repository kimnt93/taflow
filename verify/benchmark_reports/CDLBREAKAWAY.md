# CandleBreakaway benchmark (`CDLBREAKAWAY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 162.05M | 0.004 | 235.39M | 0.031 | 4.97× | 7.21× |
| 10,000 | 0.075 | 133.77M | 0.068 | 146.45M | 0.084 | 1.12× | 1.23× |
| 100,000 | 0.843 | 118.63M | 0.822 | 121.66M | 0.658 | 0.78× | 0.80× |
| 1,000,000 | 8.935 | 111.92M | 8.831 | 113.24M | 6.541 | 0.73× | 0.74× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.856 ms**; native kernel **0.844 ms**; TA-Lib 0.634 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.355 | 0.285 | 3.51M | 630.270 | 2210.62× | 94.23× |
| 100,000 | 10 | 2.587 | 1.438 | 6.96M | 640.951 | 445.81× | 18.82× |
| 100,000 | 1,000 | 26.742 | 24.440 | 40.92M | 645.630 | 26.42× | 1.21× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 89.66M | 89.86M | 1.00× | 2.28M | 2.35M | 1.00× | 122.55M |
| 2 | 189.89M | 191.97M | 2.14× | 2.35M | 2.76M | 1.17× | 131.66M |
| 4 | 328.23M | 337.22M | 3.75× | 2.35M | 2.54M | 1.08× | 127.25M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
