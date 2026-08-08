# CandleHikkake benchmark (`CDLHIKKAKE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.14M | 0.008 | 121.07M | 0.031 | 2.99× | 3.72× |
| 10,000 | 0.117 | 85.74M | 0.106 | 94.43M | 0.075 | 0.64× | 0.71× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.014 ms**; native kernel **0.012 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.383 | 0.292 | 3.42M | 32.842 | 112.44× | 96.35× |
| 1,500 | 10 | 2.656 | 1.222 | 8.18M | 33.223 | 27.18× | 23.03× |
| 1,500 | 100 | 6.384 | 3.520 | 28.41M | 34.134 | 9.70× | 8.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
