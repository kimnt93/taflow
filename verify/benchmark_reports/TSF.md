# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 61.32M | 0.017 | 59.59M | 0.043 | 2.66× | 2.59× |
| 10,000 | 0.134 | 74.58M | 0.129 | 77.48M | 0.157 | 1.17× | 1.22× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.023 ms**; TA-Lib 0.049 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.310 | 0.185 | 5.41M | 50.130 | 271.03× | 163.72× |
| 1,500 | 10 | 1.352 | 1.473 | 6.79M | 48.947 | 33.23× | 20.70× |
| 1,500 | 100 | 4.188 | 3.095 | 32.31M | 64.407 | 20.81× | 10.84× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.33M | 12.64M | 1.00× | 1.07M | 1.24M | 1.00× | 7.76M |
| 2 | 13.42M | 14.93M | 1.18× | 1.03M | 1.19M | 0.96× | 6.84M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
