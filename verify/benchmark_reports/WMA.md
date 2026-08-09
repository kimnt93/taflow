# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 151.94M | 0.006 | 181.71M | 0.033 | 4.98× | 5.95× |
| 10,000 | 0.039 | 257.95M | 0.035 | 285.05M | 0.059 | 1.53× | 1.69× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.293 | 0.166 | 6.04M | 33.395 | 201.63× | 189.03× |
| 1,500 | 10 | 1.099 | 0.594 | 16.82M | 33.447 | 56.27× | 50.83× |
| 1,500 | 100 | 3.055 | 2.018 | 49.56M | 35.148 | 17.42× | 16.06× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.74M | 8.45M | 1.00× | 1.10M | 1.48M | 1.00× | 9.26M |
| 2 | 17.39M | 21.89M | 2.59× | 1.33M | 1.55M | 1.04× | 9.78M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
