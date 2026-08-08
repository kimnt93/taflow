# CandleAbandonedBaby benchmark (`CDLABANDONEDBABY` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.53M | 0.044 | 22.66M | 0.044 | 0.98× | 0.99× |
| 10,000 | 0.454 | 22.05M | 0.425 | 23.53M | 0.147 | 0.32× | 0.34× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.072 ms**; native kernel **0.061 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.413 | 0.334 | 2.99M | 41.898 | 125.45× | 100.78× |
| 1,500 | 10 | 3.136 | 1.569 | 6.38M | 42.318 | 26.98× | 21.07× |
| 1,500 | 100 | 9.723 | 6.694 | 14.94M | 54.730 | 8.18× | 7.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
