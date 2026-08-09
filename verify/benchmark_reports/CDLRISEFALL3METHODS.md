# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.74M | 0.011 | 91.64M | 0.035 | 2.69× | 3.17× |
| 10,000 | 0.098 | 102.26M | 0.095 | 105.56M | 0.115 | 1.18× | 1.22× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.013 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.365 | 0.278 | 3.59M | 37.643 | 135.24× | 104.84× |
| 1,500 | 10 | 2.553 | 1.298 | 7.71M | 39.135 | 30.16× | 26.36× |
| 1,500 | 100 | 11.242 | 7.161 | 13.96M | 42.684 | 5.96× | 4.22× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.86M | 13.50M | 1.00× | 1.16M | 1.05M | 1.00× | 8.41M |
| 2 | 12.29M | 17.40M | 1.29× | 1.14M | 1.35M | 1.29× | 9.33M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
