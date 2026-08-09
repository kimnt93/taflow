# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 191.32M | 0.003 | 290.06M | 0.033 | 6.23× | 9.45× |
| 10,000 | 0.037 | 269.59M | 0.033 | 298.84M | 0.086 | 2.32× | 2.57× |
| 100,000 | 0.526 | 190.17M | 0.499 | 200.27M | 0.592 | 1.13× | 1.19× |
| 1,000,000 | 5.650 | 177.00M | 5.309 | 188.36M | 6.136 | 1.09× | 1.16× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.508 ms**; native kernel **0.485 ms**; TA-Lib 0.591 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.327 | 0.268 | 3.73M | 589.697 | 2200.80× | 107.24× |
| 100,000 | 10 | 2.633 | 1.491 | 6.70M | 608.604 | 408.05× | 19.08× |
| 100,000 | 1,000 | 30.223 | 36.323 | 27.53M | 599.346 | 16.50× | 0.88× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 121.66M | 134.33M | 1.00× | 1.76M | 1.76M | 1.00× | 120.25M |
| 2 | 266.99M | 283.41M | 2.11× | 2.39M | 2.61M | 1.49× | 125.96M |
| 4 | 481.72M | 552.61M | 4.11× | 2.38M | 2.64M | 1.51× | 132.04M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
