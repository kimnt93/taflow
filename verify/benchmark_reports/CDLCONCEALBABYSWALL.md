# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.20M | 0.004 | 251.42M | 0.032 | 5.57× | 8.08× |
| 10,000 | 0.052 | 191.72M | 0.049 | 204.86M | 0.089 | 1.70× | 1.82× |
| 100,000 | 0.575 | 173.80M | 0.560 | 178.68M | 0.634 | 1.10× | 1.13× |
| 1,000,000 | 6.288 | 159.04M | 6.131 | 163.11M | 6.574 | 1.05× | 1.07× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.574 ms**; native kernel **0.558 ms**; TA-Lib 0.645 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.329 | 0.273 | 3.67M | 647.686 | 2374.97× | 102.15× |
| 100,000 | 10 | 2.870 | 1.435 | 6.97M | 645.106 | 449.49× | 19.01× |
| 100,000 | 1,000 | 27.336 | 27.518 | 36.34M | 627.303 | 22.80× | 1.17× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 132.84M | 122.85M | 1.00× | 2.07M | 2.52M | 1.00× | 127.37M |
| 2 | 262.87M | 275.14M | 2.24× | 2.42M | 2.48M | 0.98× | 128.67M |
| 4 | 464.36M | 533.46M | 4.34× | 2.43M | 2.61M | 1.03× | 128.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
