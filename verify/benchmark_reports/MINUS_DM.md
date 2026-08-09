# MinusDirectionalMovement benchmark (`MINUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.48M | 0.006 | 175.16M | 0.037 | 5.24× | 6.44× |
| 10,000 | 0.054 | 183.84M | 0.050 | 201.73M | 0.080 | 1.46× | 1.61× |
| 100,000 | 0.515 | 194.32M | 0.478 | 209.16M | 0.514 | 1.00× | 1.08× |
| 1,000,000 | 5.404 | 185.05M | 4.878 | 205.02M | 4.901 | 0.91× | 1.00× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.510 ms**; native kernel **0.484 ms**; TA-Lib 0.507 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.230 | 0.178 | 5.61M | 511.580 | 2868.51× | 169.64× |
| 100,000 | 10 | 1.328 | 0.743 | 13.46M | 514.566 | 692.66× | 40.82× |
| 100,000 | 1,000 | 7.731 | 6.356 | 157.34M | 510.223 | 80.28× | 5.58× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 144.24M | 153.80M | 1.00× | 2.87M | 3.30M | 1.00× | 160.33M |
| 2 | 285.50M | 308.01M | 2.00× | 2.92M | 3.59M | 1.09× | 152.92M |
| 4 | 451.31M | 556.48M | 3.62× | 2.94M | 2.92M | 0.89× | 150.55M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
