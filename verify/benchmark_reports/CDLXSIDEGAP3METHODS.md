# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.14M | 0.010 | 95.77M | 0.031 | 2.60× | 2.99× |
| 10,000 | 0.132 | 75.82M | 0.111 | 90.04M | 0.087 | 0.66× | 0.78× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.015 ms**; native kernel **0.013 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.383 | 0.260 | 3.84M | 32.925 | 126.48× | 115.56× |
| 1,500 | 10 | 2.456 | 1.235 | 8.10M | 33.462 | 27.10× | 23.15× |
| 1,500 | 100 | 5.099 | 3.155 | 31.70M | 34.137 | 10.82× | 9.26× |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 6.84M | 7.20M | 1.00× | 849.94K | 1.32M | 1.00× | 9.34M |
| 2 | 16.37M | 17.54M | 2.43× | 1.16M | 1.42M | 1.08× | 9.00M |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
