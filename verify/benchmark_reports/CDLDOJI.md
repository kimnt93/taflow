# CandleDoji benchmark (`CDLDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.03M | 0.008 | 124.74M | 0.038 | 3.73× | 4.80× |
| 10,000 | 0.038 | 266.46M | 0.034 | 296.66M | 0.058 | 1.53× | 1.71× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.012 ms**; TA-Lib 0.052 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.499 | 0.435 | 2.30M | 40.262 | 92.63× | 105.54× |
| 1,500 | 10 | 2.679 | 1.487 | 6.73M | 37.759 | 25.40× | 21.94× |
| 1,500 | 100 | 5.382 | 2.858 | 34.99M | 43.802 | 15.32× | 12.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.07M | 13.16M | 1.00× | 1.04M | 1.23M | 1.00× | 8.24M |
| 2 | 16.77M | 16.19M | 1.23× | 1.31M | 1.29M | 1.06× | 10.39M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
