# CandleHammer benchmark (`CDLHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 95.26M | 0.009 | 115.48M | 0.039 | 3.71× | 4.49× |
| 10,000 | 0.108 | 92.92M | 0.106 | 94.11M | 0.167 | 1.55× | 1.57× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.011 ms**; TA-Lib 0.048 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.341 | 0.268 | 3.73M | 47.735 | 177.84× | 103.14× |
| 1,500 | 10 | 2.514 | 1.250 | 8.00M | 47.213 | 37.78× | 22.64× |
| 1,500 | 100 | 5.181 | 3.147 | 31.78M | 49.823 | 15.83× | 9.88× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.43M | 10.98M | 1.00× | 1.28M | 1.41M | 1.00× | 6.10M |
| 2 | 13.61M | 20.42M | 1.86× | 1.11M | 1.46M | 1.04× | 9.17M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
