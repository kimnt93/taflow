# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.068 | 14.79M | 0.028 | 36.21M | 0.044 | 0.65× | 1.58× |
| 10,000 | 0.677 | 14.76M | 0.260 | 38.53M | 0.155 | 0.23× | 0.60× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.103 ms**; native kernel **0.042 ms**; TA-Lib 0.052 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.497 | 0.396 | 2.52M | 49.279 | 124.43× | 78.02× |
| 1,500 | 10 | 2.009 | 0.938 | 10.66M | 50.789 | 54.16× | 32.52× |
| 1,500 | 100 | 9.686 | 4.563 | 21.92M | 52.130 | 11.42× | 7.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
