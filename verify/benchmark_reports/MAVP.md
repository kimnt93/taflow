# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.119 | 8.38M | 0.115 | 8.66M | 0.114 | 0.96× | 0.99× |
| 10,000 | 1.337 | 7.48M | 1.125 | 8.89M | 0.796 | 0.60× | 0.71× |
| 100,000 | 11.254 | 8.89M | 11.198 | 8.93M | 8.074 | 0.72× | 0.72× |
| 1,000,000 | 114.300 | 8.75M | 111.448 | 8.97M | 98.737 | 0.86× | 0.89× |

## Warm-up

Construct + canonical extend over 100,000 bars: **11.048 ms**; native kernel **10.923 ms**; TA-Lib 7.666 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.463 | 0.312 | 3.21M | 7478.092 | 23979.55× | 117.26× |
| 100,000 | 10 | 2.379 | 1.961 | 5.10M | 7843.793 | 3999.49× | 18.87× |
| 100,000 | 1,000 | 117.941 | 113.251 | 8.83M | 7357.099 | 64.96× | 0.99× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 10.09M | 8.04M | 1.00× | 1.76M | 2.11M | 1.00× | 12.65M |
| 2 | 17.92M | 17.87M | 2.22× | 1.70M | 2.02M | 0.96× | 12.45M |
| 4 | 33.30M | 33.93M | 4.22× | 1.70M | 1.99M | 0.95× | 12.29M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
