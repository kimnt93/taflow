# CandleTwoCrows benchmark (`CDL2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.94M | 0.013 | 74.25M | 0.031 | 1.72× | 2.28× |
| 10,000 | 0.151 | 66.39M | 0.210 | 47.56M | 0.120 | 0.80× | 0.57× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.023 ms**; native kernel **0.021 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.393 | 0.288 | 3.47M | 34.150 | 118.47× | 95.91× |
| 1,500 | 10 | 2.859 | 1.349 | 7.41M | 33.719 | 24.99× | 21.68× |
| 1,500 | 100 | 6.836 | 3.857 | 25.93M | 33.425 | 8.67× | 7.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
