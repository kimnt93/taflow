# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 105.17M | 0.007 | 135.40M | 0.036 | 3.74× | 4.82× |
| 10,000 | 0.051 | 194.31M | 0.049 | 203.01M | 0.104 | 2.02× | 2.11× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.008 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.351 | 0.270 | 3.70M | 38.847 | 143.82× | 114.84× |
| 1,500 | 10 | 2.949 | 1.303 | 7.68M | 40.283 | 30.92× | 23.17× |
| 1,500 | 100 | 5.179 | 2.886 | 34.65M | 41.273 | 14.30× | 10.87× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.09M | 14.47M | 1.00× | 866.12K | 1.16M | 1.00× | 7.87M |
| 2 | 16.02M | 19.52M | 1.35× | 1.21M | 1.28M | 1.11× | 9.39M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
