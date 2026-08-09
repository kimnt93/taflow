# CandleConcealBabySwall benchmark (`CDLCONCEALBABYSWALL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.67M | 0.004 | 248.70M | 0.037 | 6.40× | 9.16× |
| 10,000 | 0.048 | 208.24M | 0.044 | 225.28M | 0.085 | 1.78× | 1.92× |
| 100,000 | 0.545 | 183.59M | 0.525 | 190.41M | 0.624 | 1.15× | 1.19× |
| 1,000,000 | 5.901 | 169.47M | 5.841 | 171.21M | 6.195 | 1.05× | 1.06× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.540 ms**; native kernel **0.531 ms**; TA-Lib 0.627 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.321 | 0.262 | 3.82M | 617.575 | 2360.77× | 104.35× |
| 100,000 | 10 | 2.504 | 1.313 | 7.62M | 622.673 | 474.27× | 20.70× |
| 100,000 | 1,000 | 29.636 | 25.670 | 38.96M | 630.987 | 24.58× | 1.25× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 132.55M | 151.54M | 1.00× | 2.08M | 2.59M | 1.00× | 122.36M |
| 2 | 264.91M | 289.83M | 1.91× | 2.47M | 2.61M | 1.01× | 127.65M |
| 4 | 421.59M | 525.71M | 3.47× | 2.28M | 2.49M | 0.96× | 125.79M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
