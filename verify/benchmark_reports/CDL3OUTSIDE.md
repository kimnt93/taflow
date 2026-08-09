# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.99M | 0.006 | 160.72M | 0.030 | 3.65× | 4.81× |
| 10,000 | 0.067 | 148.82M | 0.063 | 158.12M | 0.081 | 1.20× | 1.27× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.009 ms**; native kernel **0.007 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.341 | 0.264 | 3.79M | 31.610 | 119.93× | 106.32× |
| 1,500 | 10 | 2.573 | 1.276 | 7.84M | 32.490 | 25.46× | 21.41× |
| 1,500 | 100 | 4.719 | 2.796 | 35.77M | 31.818 | 11.38× | 9.77× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.29M | 17.25M | 1.00× | 1.12M | 1.03M | 1.00× | 7.85M |
| 2 | 15.18M | 20.01M | 1.16× | 1.25M | 1.23M | 1.19× | 8.97M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
