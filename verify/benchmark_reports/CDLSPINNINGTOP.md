# CandleSpinningTop benchmark (`CDLSPINNINGTOP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 102.70M | 0.008 | 126.07M | 0.035 | 3.55× | 4.35× |
| 10,000 | 0.103 | 97.48M | 0.103 | 97.26M | 0.125 | 1.22× | 1.22× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.011 ms**; native kernel **0.009 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.361 | 0.325 | 3.08M | 34.815 | 107.15× | 89.91× |
| 1,500 | 10 | 2.557 | 1.228 | 8.14M | 34.305 | 27.93× | 24.02× |
| 1,500 | 100 | 5.194 | 2.760 | 36.23M | 34.605 | 12.54× | 10.82× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 7.43M | 11.62M | 1.00× | 791.05K | 1.23M | 1.00× | 8.84M |
| 2 | 14.05M | 18.15M | 1.56× | 1.32M | 1.29M | 1.06× | 9.47M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
