# CandleKickingByLength benchmark (`CDLKICKINGBYLENGTH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.55M | 0.010 | 100.70M | 0.039 | 3.37× | 3.96× |
| 10,000 | 0.084 | 118.87M | 0.080 | 124.61M | 0.179 | 2.13× | 2.23× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.012 ms**; TA-Lib 0.046 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.382 | 0.281 | 3.56M | 45.595 | 162.47× | 99.23× |
| 1,500 | 10 | 2.533 | 1.285 | 7.78M | 45.825 | 35.66× | 22.06× |
| 1,500 | 100 | 5.636 | 3.356 | 29.80M | 47.836 | 14.25× | 8.98× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 11.56M | 15.07M | 1.00× | 954.26K | 1.42M | 1.00× | 8.19M |
| 2 | 18.13M | 16.46M | 1.09× | 1.07M | 1.42M | 1.00× | 6.98M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
