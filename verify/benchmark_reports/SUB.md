# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 386.31M | 0.001 | 935.78M | 0.031 | 11.80× | 28.60× |
| 10,000 | 0.008 | 1.28G | 0.004 | 2.33G | 0.040 | 5.15× | 9.40× |
| 100,000 | 0.063 | 1.60G | 0.037 | 2.68G | 0.066 | 1.06× | 1.77× |
| 1,000,000 | 1.261 | 793.12M | 0.920 | 1.09G | 0.948 | 0.75× | 1.03× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.060 ms**; native kernel **0.039 ms**; TA-Lib 0.067 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.282 | 0.209 | 4.80M | 65.716 | 315.18× | 152.03× |
| 100,000 | 10 | 2.050 | 1.014 | 9.86M | 91.355 | 90.08× | 36.86× |
| 100,000 | 1,000 | 7.406 | 2.815 | 355.19M | 79.380 | 28.19× | 11.97× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 518.00M | 907.43M | 1.00× | 2.90M | 3.43M | 1.00× | 594.23M |
| 2 | 791.02M | 1.22G | 1.35× | 2.37M | 2.99M | 0.87× | 521.36M |
| 4 | 717.60M | 1.44G | 1.58× | 2.55M | 2.63M | 0.77× | 475.70M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
