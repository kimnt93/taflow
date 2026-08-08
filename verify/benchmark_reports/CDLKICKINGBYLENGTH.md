# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MISMATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.74M | 0.006 | 170.93M | 0.039 | 4.92× | 6.59× |
| 10,000 | 0.073 | 136.87M | 0.069 | 144.26M | 0.188 | 2.57× | 2.71× |
| 100,000 | 0.987 | 101.31M | 0.976 | 102.44M | 1.612 | 1.63× | 1.65× |
| 1,000,000 | 10.532 | 94.95M | 10.381 | 96.33M | 16.153 | 1.53× | 1.56× |

## Warm-up

Construct + canonical extend over 100,000 bars: **0.998 ms**; native kernel **0.965 ms**; TA-Lib 1.607 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.403 | 0.295 | 3.39M | 1630.877 | 5523.48× | 98.32× |
| 100,000 | 10 | 2.851 | 1.438 | 6.96M | 1567.158 | 1090.04× | 20.35× |
| 100,000 | 1,000 | 31.169 | 28.821 | 34.70M | 1550.527 | 53.80× | 1.38× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 86.25M | 97.54M | 1.00× | 2.11M | 2.39M | 1.00× | 60.79M |
| 2 | 174.87M | 178.13M | 1.83× | 2.41M | 2.66M | 1.11× | 57.05M |
| 4 | 325.35M | 327.80M | 3.36× | 2.16M | 2.36M | 0.99× | 59.35M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
