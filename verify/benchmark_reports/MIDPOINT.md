# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 146.14M | 0.006 | 170.06M | 0.035 | 5.05× | 5.87× |
| 10,000 | 0.075 | 133.07M | 0.072 | 138.71M | 0.097 | 1.29× | 1.34× |
| 100,000 | 0.767 | 130.34M | 0.737 | 135.71M | 0.671 | 0.87× | 0.91× |
| 1,000,000 | 8.714 | 114.76M | 8.294 | 120.57M | 6.675 | 0.77× | 0.80× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.757 ms**; native kernel **0.733 ms**; TA-Lib 0.679 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.240 | 0.173 | 5.77M | 671.313 | 3873.47× | 173.18× |
| 100,000 | 10 | 1.259 | 0.850 | 11.77M | 672.441 | 791.16× | 34.88× |
| 100,000 | 1,000 | 28.080 | 24.341 | 41.08M | 689.198 | 28.31× | 1.49× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 107.20M | 114.30M | 1.00× | 2.84M | 3.07M | 1.00× | 126.38M |
| 2 | 206.38M | 212.98M | 1.86× | 3.09M | 3.33M | 1.08× | 118.21M |
| 4 | 277.37M | 331.44M | 2.90× | 2.59M | 2.83M | 0.92× | 118.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
