# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 172.72M | 0.005 | 207.96M | 0.035 | 6.04× | 7.28× |
| 10,000 | 0.030 | 328.53M | 0.027 | 363.66M | 0.060 | 1.96× | 2.17× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.007 ms**; native kernel **0.006 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.245 | 0.164 | 6.09M | 36.514 | 222.41× | 197.33× |
| 1,500 | 10 | 1.049 | 0.622 | 16.08M | 37.399 | 60.14× | 53.28× |
| 1,500 | 100 | 4.772 | 3.359 | 29.77M | 49.640 | 14.78× | 9.67× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.19M | 11.84M | 1.00× | 1.40M | 1.33M | 1.00× | 8.83M |
| 2 | 19.88M | 21.96M | 1.86× | 1.53M | 1.52M | 1.14× | 10.32M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
