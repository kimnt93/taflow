# CandleHikkakeModified benchmark (`CDLHIKKAKEMOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.65M | 0.010 | 99.88M | 0.033 | 2.45× | 3.33× |
| 10,000 | 0.067 | 148.59M | 0.061 | 162.73M | 0.085 | 1.26× | 1.38× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.013 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.347 | 0.269 | 3.72M | 35.685 | 132.71× | 105.84× |
| 1,500 | 10 | 2.656 | 1.351 | 7.40M | 35.664 | 26.39× | 24.95× |
| 1,500 | 100 | 5.121 | 3.020 | 33.11M | 37.590 | 12.45× | 9.80× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.23M | 10.12M | 1.00× | 1.13M | 932.99K | 1.00× | 9.45M |
| 2 | 14.85M | 21.15M | 2.09× | 1.38M | 1.40M | 1.50× | 9.71M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
