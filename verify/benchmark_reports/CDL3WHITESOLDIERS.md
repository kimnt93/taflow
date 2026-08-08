# CandleThreeWhiteSoldiers benchmark (`CDL3WHITESOLDIERS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.80M | 0.056 | 17.71M | 0.043 | 0.73× | 0.77× |
| 10,000 | 0.586 | 17.06M | 0.571 | 17.51M | 0.192 | 0.33× | 0.34× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.087 ms**; native kernel **0.086 ms**; TA-Lib 0.054 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.455 | 0.369 | 2.71M | 54.500 | 147.85× | 76.09× |
| 1,500 | 10 | 3.235 | 1.786 | 5.60M | 53.337 | 29.86× | 15.93× |
| 1,500 | 100 | 10.867 | 8.108 | 12.33M | 56.014 | 6.91× | 3.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
