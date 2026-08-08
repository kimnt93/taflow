# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.33M | 0.068 | 14.67M | 0.078 | 1.12× | 1.15× |
| 10,000 | 0.718 | 13.93M | 0.709 | 14.10M | 0.616 | 0.86× | 0.87× |
| 100,000 | 7.459 | 13.41M | 7.023 | 14.24M | 5.785 | 0.78× | 0.82× |
| 1,000,000 | 70.274 | 14.23M | 69.847 | 14.32M | 57.490 | 0.82× | 0.82× |

## Warm-up

Construct + canonical extend over 100,000 bars: **6.983 ms**; native kernel **7.042 ms**; TA-Lib 5.702 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.285 | 0.229 | 4.36M | 5817.890 | 25394.68× | 120.84× |
| 100,000 | 10 | 1.263 | 1.196 | 8.36M | 5708.674 | 4773.54× | 23.38× |
| 100,000 | 1,000 | 81.110 | 69.630 | 14.36M | 5879.000 | 84.43× | 1.18× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 13.57M | 13.81M | 1.00× | 2.34M | 3.04M | 1.00× | 16.57M |
| 2 | 25.95M | 26.34M | 1.91× | 2.30M | 2.64M | 0.87× | 16.47M |
| 4 | 49.82M | 48.72M | 3.53× | 2.23M | 2.32M | 0.76× | 16.22M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
