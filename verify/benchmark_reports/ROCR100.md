# RateOfChangeRatioPercent benchmark (`ROCR100` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 198.12M | 0.005 | 213.25M | 0.032 | 6.28× | 6.76× |
| 10,000 | 0.023 | 428.50M | 0.021 | 471.61M | 0.041 | 1.77× | 1.95× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.282 | 0.160 | 6.25M | 31.425 | 196.51× | 199.26× |
| 1,500 | 10 | 1.137 | 0.591 | 16.91M | 32.409 | 54.80× | 51.04× |
| 1,500 | 100 | 3.009 | 1.822 | 54.89M | 32.446 | 17.81× | 16.31× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.17M | 16.76M | 1.00× | 1.35M | 1.43M | 1.00× | 9.88M |
| 2 | 19.46M | 17.81M | 1.06× | 1.22M | 1.28M | 0.90× | 9.83M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
