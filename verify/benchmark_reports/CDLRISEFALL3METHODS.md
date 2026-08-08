# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.058 | 17.16M | 0.057 | 17.50M | 0.036 | 0.62× | 0.64× |
| 10,000 | 0.590 | 16.95M | 0.608 | 16.44M | 0.124 | 0.21× | 0.20× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.091 ms**; native kernel **0.091 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.435 | 0.348 | 2.88M | 38.904 | 111.93× | 84.40× |
| 1,500 | 10 | 4.722 | 1.808 | 5.53M | 39.053 | 21.60× | 17.29× |
| 1,500 | 100 | 11.728 | 9.094 | 11.00M | 40.457 | 4.45× | 3.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
