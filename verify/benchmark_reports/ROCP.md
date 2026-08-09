# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.47M | 0.005 | 215.29M | 0.033 | 5.03× | 7.20× |
| 10,000 | 0.025 | 402.67M | 0.020 | 490.58M | 0.041 | 1.67× | 2.03× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.006 ms**; native kernel **0.005 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.294 | 0.168 | 5.97M | 31.167 | 186.01× | 171.60× |
| 1,500 | 10 | 1.037 | 0.542 | 18.44M | 31.734 | 58.52× | 55.52× |
| 1,500 | 100 | 3.036 | 1.877 | 53.29M | 32.101 | 17.11× | 15.96× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.41M | 17.73M | 1.00× | 1.38M | 1.46M | 1.00× | 9.22M |
| 2 | 9.65M | 23.03M | 1.30× | 1.17M | 1.48M | 1.01× | 7.90M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
