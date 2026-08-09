# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.163 | 6.14M | 0.163 | 6.15M | 0.615 | 3.78× | 3.78× |
| 10,000 | 1.677 | 5.96M | 1.692 | 5.91M | 5.888 | 3.51× | 3.48× |
| 100,000 | 16.859 | 5.93M | 16.892 | 5.92M | 61.034 | 3.62× | 3.61× |
| 1,000,000 | 169.728 | 5.89M | 171.828 | 5.82M | 586.027 | 3.45× | 3.41× |

## Warm-up

Construct + canonical extend over 100,000 bars: **16.886 ms**; native kernel **17.004 ms**; TA-Lib 59.092 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.413 | 0.349 | 2.86M | 57722.347 | 165344.75× | 103.93× |
| 100,000 | 10 | 2.808 | 2.539 | 3.94M | 58702.169 | 23117.08× | 16.22× |
| 100,000 | 1,000 | 175.688 | 177.980 | 5.62M | 59126.087 | 332.21× | 3.59× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 5.88M | 5.84M | 1.00× | 1.81M | 2.09M | 1.00× | 2.20M |
| 2 | 11.17M | 11.18M | 1.91× | 1.97M | 2.01M | 0.96× | 2.11M |
| 4 | 21.31M | 20.01M | 3.43× | 1.72M | 2.00M | 0.95× | 2.17M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
