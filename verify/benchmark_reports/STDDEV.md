# RollingStandardDeviation benchmark (`STDDEV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.79M | 0.008 | 120.71M | 0.036 | 0.71× | 4.34× |
| 10,000 | 0.477 | 20.96M | 0.077 | 130.09M | 0.061 | 0.13× | 0.80× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.077 ms**; native kernel **0.012 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.332 | 0.203 | 4.92M | 38.057 | 187.39× | 156.12× |
| 1,500 | 10 | 1.829 | 0.724 | 13.82M | 36.391 | 50.28× | 46.48× |
| 1,500 | 100 | 8.183 | 2.938 | 34.04M | 38.793 | 13.20× | 11.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
