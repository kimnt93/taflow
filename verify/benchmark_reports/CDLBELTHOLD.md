# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 80.28M | 0.010 | 97.99M | 0.037 | 2.94× | 3.59× |
| 10,000 | 0.146 | 68.40M | 0.142 | 70.53M | 0.136 | 0.93× | 0.96× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.022 ms**; native kernel **0.015 ms**; TA-Lib 0.044 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.395 | 0.307 | 3.26M | 41.883 | 136.63× | 90.96× |
| 1,500 | 10 | 2.762 | 1.323 | 7.56M | 41.893 | 31.66× | 21.34× |
| 1,500 | 100 | 7.135 | 4.136 | 24.18M | 44.848 | 10.84× | 7.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
