# CandleStickSandwich benchmark (`CDLSTICKSANDWICH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.56M | 0.009 | 108.02M | 0.032 | 2.87× | 3.46× |
| 10,000 | 0.120 | 83.44M | 0.117 | 85.37M | 0.090 | 0.76× | 0.77× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.016 ms**; native kernel **0.013 ms**; TA-Lib 0.034 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.367 | 0.288 | 3.47M | 33.832 | 117.51× | 98.12× |
| 1,500 | 10 | 2.599 | 1.214 | 8.24M | 35.598 | 29.32× | 23.63× |
| 1,500 | 100 | 6.411 | 3.553 | 28.15M | 36.325 | 10.22× | 8.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
