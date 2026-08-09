# CandleLongLeggedDoji benchmark (`CDLLONGLEGGEDDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.47M | 0.008 | 128.74M | 0.034 | 3.56× | 4.39× |
| 10,000 | 0.053 | 188.24M | 0.051 | 196.49M | 0.092 | 1.74× | 1.82× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.347 | 0.281 | 3.56M | 36.990 | 131.52× | 100.94× |
| 1,500 | 10 | 2.556 | 1.471 | 6.80M | 39.483 | 26.83× | 19.40× |
| 1,500 | 100 | 5.282 | 2.761 | 36.21M | 37.770 | 13.68× | 10.52× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.69M | 14.40M | 1.00× | 1.18M | 1.12M | 1.00× | 8.48M |
| 2 | 16.01M | 18.83M | 1.31× | 1.27M | 1.48M | 1.32× | 9.56M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
