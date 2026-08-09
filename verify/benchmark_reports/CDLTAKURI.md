# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 104.46M | 0.008 | 131.57M | 0.037 | 3.81× | 4.80× |
| 10,000 | 0.055 | 182.86M | 0.051 | 195.17M | 0.108 | 1.98× | 2.11× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.349 | 0.267 | 3.74M | 41.989 | 157.01× | 117.17× |
| 1,500 | 10 | 4.779 | 2.268 | 4.41M | 42.171 | 18.59× | 12.87× |
| 1,500 | 100 | 5.330 | 3.179 | 31.45M | 42.456 | 13.35× | 10.03× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.10M | 10.89M | 1.00× | 1.19M | 1.10M | 1.00× | 9.00M |
| 2 | 12.57M | 17.64M | 1.62× | 1.29M | 1.45M | 1.31× | 9.84M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
