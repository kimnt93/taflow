# CandleUpsideGapTwoCrows benchmark (`CDLUPSIDEGAP2CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.02M | 0.027 | 37.18M | 0.033 | 1.12× | 1.23× |
| 10,000 | 0.347 | 28.78M | 0.291 | 34.33M | 0.124 | 0.36× | 0.42× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.044 ms**; native kernel **0.040 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.391 | 0.315 | 3.18M | 35.245 | 111.90× | 95.52× |
| 1,500 | 10 | 2.758 | 1.414 | 7.07M | 34.735 | 24.56× | 20.46× |
| 1,500 | 100 | 8.227 | 5.300 | 18.87M | 36.239 | 6.84× | 5.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
