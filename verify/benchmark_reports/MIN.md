# RollingMin benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 229.13M | 0.003 | 290.06M | 0.035 | 8.01× | 10.14× |
| 10,000 | 0.026 | 386.32M | 0.023 | 427.76M | 0.081 | 3.12× | 3.46× |
| 100,000 | 0.232 | 431.38M | 0.207 | 482.88M | 0.528 | 2.28× | 2.55× |
| 1,000,000 | 2.956 | 338.27M | 2.497 | 400.52M | 5.231 | 1.77× | 2.10× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.232 ms**; native kernel **0.212 ms**; TA-Lib 0.568 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.248 | 0.171 | 5.84M | 537.963 | 3139.77× | 188.82× |
| 100,000 | 10 | 1.070 | 0.625 | 15.99M | 522.931 | 836.20× | 48.80× |
| 100,000 | 1,000 | 13.986 | 14.839 | 67.39M | 540.187 | 36.40× | 2.50× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 232.49M | 295.34M | 1.00× | 2.64M | 3.96M | 1.00× | 156.93M |
| 2 | 519.58M | 574.37M | 1.94× | 2.93M | 3.96M | 1.00× | 151.13M |
| 4 | 599.37M | 872.09M | 2.95× | 2.99M | 3.42M | 0.86× | 157.44M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
