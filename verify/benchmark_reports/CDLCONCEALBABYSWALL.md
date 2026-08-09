# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 116.86M | 0.008 | 131.31M | 0.032 | 3.72× | 4.18× |
| 10,000 | 0.050 | 198.06M | 0.049 | 203.17M | 0.109 | 2.16× | 2.22× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.010 ms**; native kernel **0.009 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.505 | 0.269 | 3.71M | 38.096 | 141.36× | 108.74× |
| 1,500 | 10 | 2.801 | 1.385 | 7.22M | 37.688 | 27.22× | 21.46× |
| 1,500 | 100 | 6.201 | 3.886 | 25.74M | 37.460 | 9.64× | 7.47× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.98M | 12.00M | 1.00× | 1.19M | 1.17M | 1.00× | 8.87M |
| 2 | 18.01M | 17.36M | 1.45× | 1.31M | 1.43M | 1.21× | 10.08M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
