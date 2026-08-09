# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.068 | 14.62M | 0.067 | 14.92M | 0.078 | 1.14× | 1.17× |
| 10,000 | 0.703 | 14.22M | 0.695 | 14.38M | 0.586 | 0.83× | 0.84× |
| 100,000 | 7.064 | 14.16M | 6.973 | 14.34M | 5.724 | 0.81× | 0.82× |
| 1,000,000 | 70.333 | 14.22M | 70.076 | 14.27M | 57.449 | 0.82× | 0.82× |

## Warm-up

Construct + canonical extend over 100,000 bars: **7.031 ms**; native kernel **6.858 ms**; TA-Lib 5.586 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.273 | 0.221 | 4.52M | 5885.927 | 26597.19× | 124.75× |
| 100,000 | 10 | 1.364 | 1.285 | 7.78M | 5619.161 | 4374.05× | 21.89× |
| 100,000 | 1,000 | 72.937 | 69.361 | 14.42M | 5667.306 | 81.71× | 1.14× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.74M | 14.02M | 1.00× | 2.62M | 2.79M | 1.00× | 16.51M |
| 2 | 26.73M | 26.75M | 1.91× | 2.29M | 2.51M | 0.90× | 16.46M |
| 4 | 49.29M | 50.87M | 3.63× | 2.20M | 2.10M | 0.75× | 16.15M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
