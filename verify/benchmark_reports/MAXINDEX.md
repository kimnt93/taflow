# RollingArgmax benchmark (`MAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 155.88M | 0.005 | 182.08M | 0.035 | 5.45× | 6.36× |
| 10,000 | 0.055 | 182.72M | 0.051 | 197.97M | 0.096 | 1.75× | 1.90× |
| 100,000 | 0.524 | 190.79M | 0.506 | 197.57M | 0.676 | 1.29× | 1.34× |
| 1,000,000 | 5.377 | 185.98M | 5.262 | 190.03M | 6.573 | 1.22× | 1.25× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.526 ms**; native kernel **0.492 ms**; TA-Lib 0.691 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.234 | 0.164 | 6.08M | 674.515 | 4103.09× | 170.50× |
| 100,000 | 10 | 0.975 | 0.643 | 15.55M | 678.801 | 1055.57× | 43.89× |
| 100,000 | 1,000 | 14.439 | 11.969 | 83.55M | 680.527 | 56.86× | 2.92× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 156.09M | 183.55M | 1.00× | 3.50M | 4.09M | 1.00× | 126.43M |
| 2 | 264.58M | 318.68M | 1.74× | 2.98M | 3.58M | 0.88× | 123.67M |
| 4 | 424.61M | 541.52M | 2.95× | 2.77M | 2.93M | 0.72× | 120.33M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
