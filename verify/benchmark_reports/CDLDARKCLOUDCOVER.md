# CandleDarkCloudCover benchmark (`CDLDARKCLOUDCOVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.83M | 0.004 | 285.45M | 0.033 | 6.32× | 9.55× |
| 10,000 | 0.068 | 146.93M | 0.064 | 156.13M | 0.107 | 1.57× | 1.67× |
| 100,000 | 0.814 | 122.89M | 0.814 | 122.92M | 0.813 | 1.00× | 1.00× |
| 1,000,000 | 8.673 | 115.31M | 8.531 | 117.22M | 7.851 | 0.91× | 0.92× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.821 ms**; native kernel **0.811 ms**; TA-Lib 0.806 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.332 | 0.263 | 3.80M | 803.037 | 3053.15× | 124.43× |
| 100,000 | 10 | 2.987 | 1.372 | 7.29M | 794.526 | 579.05× | 22.36× |
| 100,000 | 1,000 | 26.664 | 23.637 | 42.31M | 806.122 | 34.10× | 1.43× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 99.44M | 104.11M | 1.00× | 2.09M | 2.78M | 1.00× | 106.69M |
| 2 | 196.97M | 198.97M | 1.91× | 2.47M | 2.80M | 1.01× | 99.44M |
| 4 | 326.20M | 358.63M | 3.44× | 2.33M | 2.55M | 0.91× | 104.84M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
