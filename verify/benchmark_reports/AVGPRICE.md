# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.71M | 0.005 | 189.91M | 0.029 | 3.96× | 5.46× |
| 10,000 | 0.016 | 641.96M | 0.011 | 871.29M | 0.036 | 2.32× | 3.15× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.008 ms**; native kernel **0.006 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.389 | 0.251 | 3.98M | 30.200 | 120.17× | 115.50× |
| 1,500 | 10 | 3.515 | 1.187 | 8.43M | 29.568 | 24.91× | 23.88× |
| 1,500 | 100 | 4.666 | 2.406 | 41.57M | 29.710 | 12.35× | 11.57× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.08M | 9.13M | 1.00× | 1.03M | 1.29M | 1.00× | 7.10M |
| 2 | 12.95M | 18.76M | 2.05× | 1.11M | 1.47M | 1.14× | 10.62M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
