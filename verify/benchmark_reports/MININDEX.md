# RollingArgmin benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 179.98M | 0.004 | 224.87M | 0.034 | 6.19× | 7.74× |
| 10,000 | 0.051 | 196.48M | 0.048 | 209.62M | 0.092 | 1.80× | 1.92× |
| 100,000 | 0.523 | 191.22M | 0.497 | 201.39M | 0.670 | 1.28× | 1.35× |
| 1,000,000 | 5.236 | 190.98M | 5.050 | 198.02M | 6.719 | 1.28× | 1.33× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.509 ms**; native kernel **0.491 ms**; TA-Lib 0.686 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.224 | 0.162 | 6.19M | 666.789 | 4124.36× | 174.88× |
| 100,000 | 10 | 0.995 | 0.621 | 16.11M | 674.322 | 1086.05× | 46.97× |
| 100,000 | 1,000 | 18.350 | 15.482 | 64.59M | 715.203 | 46.19× | 2.29× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 139.22M | 165.85M | 1.00× | 3.02M | 3.41M | 1.00× | 120.71M |
| 2 | 278.70M | 297.55M | 1.79× | 2.91M | 3.79M | 1.11× | 116.49M |
| 4 | 420.17M | 506.29M | 3.05× | 2.83M | 3.12M | 0.91× | 119.64M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
