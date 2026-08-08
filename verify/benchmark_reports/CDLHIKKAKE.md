# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.81M | 0.008 | 130.91M | 0.038 | 3.81× | 4.94× |
| 10,000 | 0.107 | 93.67M | 0.114 | 87.83M | 0.077 | 0.72× | 0.67× |
| 100,000 | 1.001 | 99.93M | 1.005 | 99.50M | 0.514 | 0.51× | 0.51× |
| 1,000,000 | 10.633 | 94.05M | 10.479 | 95.43M | 5.099 | 0.48× | 0.49× |

## Warm-up

Construct + canonical extend over 100,000 bars: **1.049 ms**; native kernel **1.168 ms**; TA-Lib 0.501 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.335 | 0.285 | 3.51M | 502.577 | 1763.68× | 97.99× |
| 100,000 | 10 | 2.671 | 1.456 | 6.87M | 516.823 | 354.90× | 19.19× |
| 100,000 | 1,000 | 37.424 | 25.807 | 38.75M | 510.358 | 19.78× | 1.17× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 80.85M | 89.41M | 1.00× | 2.12M | 2.42M | 1.00× | 156.19M |
| 2 | 165.70M | 156.74M | 1.75× | 2.05M | 2.51M | 1.04× | 159.18M |
| 4 | 254.31M | 284.79M | 3.19× | 2.18M | 2.39M | 0.99× | 154.92M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
