# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 144.64M | 0.005 | 196.93M | 0.036 | 5.17× | 7.04× |
| 10,000 | 0.067 | 150.37M | 0.063 | 159.58M | 0.125 | 1.88× | 2.00× |
| 100,000 | 0.732 | 136.69M | 0.717 | 139.52M | 0.974 | 1.33× | 1.36× |
| 1,000,000 | 7.676 | 130.28M | 7.630 | 131.05M | 9.678 | 1.26× | 1.27× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.740 ms**; native kernel **0.715 ms**; TA-Lib 0.971 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.341 | 0.273 | 3.66M | 968.508 | 3544.35× | 103.79× |
| 100,000 | 10 | 2.524 | 1.351 | 7.40M | 978.528 | 724.09× | 20.41× |
| 100,000 | 1,000 | 27.678 | 23.952 | 41.75M | 976.954 | 40.79× | 1.49× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 111.49M | 115.89M | 1.00× | 2.42M | 2.32M | 1.00× | 89.76M |
| 2 | 233.54M | 234.31M | 2.02× | 2.38M | 2.54M | 1.09× | 90.24M |
| 4 | 382.27M | 392.20M | 3.38× | 2.46M | 2.53M | 1.09× | 89.04M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
