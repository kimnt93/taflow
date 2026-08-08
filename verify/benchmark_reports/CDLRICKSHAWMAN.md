# CandleRickshawman benchmark (`CDLRICKSHAWMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.91M | 0.011 | 93.60M | 0.038 | 2.98× | 3.53× |
| 10,000 | 0.113 | 88.75M | 0.173 | 57.77M | 0.128 | 1.13× | 0.74× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.017 ms**; native kernel **0.015 ms**; TA-Lib 0.042 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.365 | 0.283 | 3.54M | 42.171 | 149.08× | 99.44× |
| 1,500 | 10 | 2.681 | 1.256 | 7.96M | 41.243 | 32.85× | 24.14× |
| 1,500 | 100 | 6.877 | 3.658 | 27.34M | 44.469 | 12.16× | 8.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
