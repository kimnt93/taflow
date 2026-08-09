# CandleShootingStar benchmark (`CDLSHOOTINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.71M | 0.008 | 123.24M | 0.041 | 4.10× | 5.02× |
| 10,000 | 0.103 | 97.38M | 0.101 | 99.29M | 0.169 | 1.65× | 1.68× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.047 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.368 | 0.272 | 3.67M | 46.902 | 172.35× | 107.46× |
| 1,500 | 10 | 2.503 | 1.240 | 8.07M | 59.357 | 47.89× | 23.13× |
| 1,500 | 100 | 5.869 | 3.152 | 31.72M | 48.777 | 15.47× | 9.45× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 8.69M | 9.35M | 1.00× | 853.70K | 1.25M | 1.00× | 7.10M |
| 2 | 15.65M | 18.92M | 2.02× | 1.27M | 1.39M | 1.11× | 9.40M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
