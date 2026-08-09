# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.90M | 0.004 | 236.43M | 0.033 | 5.82× | 7.91× |
| 10,000 | 0.037 | 272.59M | 0.034 | 295.06M | 0.050 | 1.35× | 1.46× |
| 100,000 | 0.348 | 287.50M | 0.361 | 276.80M | 0.217 | 0.62× | 0.60× |
| 1,000,000 | 3.671 | 272.43M | 3.449 | 289.95M | 2.002 | 0.55× | 0.58× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.346 ms**; native kernel **0.321 ms**; TA-Lib 0.216 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.210 | 0.145 | 6.89M | 217.488 | 1498.58× | 206.43× |
| 100,000 | 10 | 0.938 | 0.494 | 20.26M | 214.239 | 433.97× | 60.43× |
| 100,000 | 1,000 | 5.915 | 4.435 | 225.50M | 219.664 | 49.53× | 7.37× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 164.30M | 232.72M | 1.00× | 3.42M | 3.32M | 1.00× | 301.76M |
| 2 | 411.94M | 415.11M | 1.78× | 2.99M | 3.96M | 1.19× | 333.70M |
| 4 | 508.24M | 847.44M | 3.64× | 3.06M | 3.22M | 0.97× | 317.07M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
