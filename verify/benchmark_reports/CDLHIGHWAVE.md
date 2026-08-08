# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.04M | 0.009 | 110.14M | 0.035 | 3.05× | 3.81× |
| 10,000 | 0.144 | 69.53M | 0.144 | 69.58M | 0.160 | 1.11× | 1.11× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.015 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.381 | 0.284 | 3.53M | 43.845 | 154.64× | 98.71× |
| 1,500 | 10 | 2.742 | 1.267 | 7.89M | 43.292 | 34.17× | 22.36× |
| 1,500 | 100 | 6.521 | 3.906 | 25.60M | 46.010 | 11.78× | 7.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
