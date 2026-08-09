# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.125 | 8.00M | 0.113 | 8.88M | 0.106 | 0.85× | 0.94× |
| 10,000 | 1.277 | 7.83M | 1.092 | 9.16M | 0.757 | 0.59× | 0.69× |
| 100,000 | 11.700 | 8.55M | 10.964 | 9.12M | 7.164 | 0.61× | 0.65× |
| 1,000,000 | 117.414 | 8.52M | 109.121 | 9.16M | 91.663 | 0.78× | 0.84× |

## Warm-up

Construct + canonical extend over 100,000 bars: **11.674 ms**; native kernel **11.019 ms**; TA-Lib 6.964 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.433 | 0.310 | 3.22M | 7189.544 | 23184.26× | 119.53× |
| 100,000 | 10 | 2.574 | 2.027 | 4.93M | 7432.196 | 3666.71× | 18.27× |
| 100,000 | 1,000 | 117.900 | 110.409 | 9.06M | 7246.087 | 65.63× | 0.98× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 9.01M | 9.20M | 1.00× | 1.55M | 2.03M | 1.00× | 12.31M |
| 2 | 17.57M | 18.05M | 1.96× | 1.60M | 1.74M | 0.85× | 12.47M |
| 4 | 33.49M | 34.34M | 3.73× | 1.46M | 1.86M | 0.91× | 12.39M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
