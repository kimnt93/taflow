# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.164 | 6.11M | 0.165 | 6.07M | 0.465 | 2.84× | 2.82× |
| 10,000 | 1.660 | 6.02M | 1.636 | 6.11M | 4.752 | 2.86× | 2.91× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.259 ms**; native kernel **0.246 ms**; TA-Lib 0.691 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.411 | 0.334 | 2.99M | 692.843 | 2074.80× | 102.42× |
| 1,500 | 10 | 2.767 | 2.245 | 4.45M | 686.235 | 305.61× | 17.68× |
| 1,500 | 100 | 18.883 | 19.142 | 5.22M | 728.644 | 38.07× | 4.50× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 3.37M | 3.76M | 1.00× | 911.65K | 1.17M | 1.00× | 1.59M |
| 2 | 6.73M | 6.53M | 1.73× | 999.99K | 1.05M | 0.90× | 1.62M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
