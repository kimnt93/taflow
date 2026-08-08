# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.54M | 0.030 | 33.86M | 0.036 | 1.16× | 1.20× |
| 10,000 | 0.337 | 29.64M | 0.323 | 30.99M | 0.143 | 0.42× | 0.44× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.045 ms**; native kernel **0.044 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.397 | 0.311 | 3.22M | 40.939 | 131.68× | 90.54× |
| 1,500 | 10 | 2.867 | 1.420 | 7.04M | 39.531 | 27.85× | 19.92× |
| 1,500 | 100 | 8.985 | 5.649 | 17.70M | 42.888 | 7.59× | 5.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
