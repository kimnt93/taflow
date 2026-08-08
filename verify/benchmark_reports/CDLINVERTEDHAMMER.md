# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.028 | 35.41M | 0.025 | 39.52M | 0.041 | 1.46× | 1.63× |
| 10,000 | 0.290 | 34.44M | 0.285 | 35.11M | 0.175 | 0.60× | 0.62× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.041 ms**; native kernel **0.039 ms**; TA-Lib 0.049 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.402 | 0.320 | 3.13M | 46.998 | 146.90× | 89.32× |
| 1,500 | 10 | 2.813 | 1.386 | 7.21M | 46.252 | 33.36× | 20.62× |
| 1,500 | 100 | 8.263 | 5.300 | 18.87M | 50.186 | 9.47× | 5.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
