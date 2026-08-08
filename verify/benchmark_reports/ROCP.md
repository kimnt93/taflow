# RateOfChangePercent benchmark (`ROCP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.47M | 0.004 | 246.36M | 0.034 | 0.67× | 8.43× |
| 10,000 | 0.474 | 21.11M | 0.032 | 309.30M | 0.041 | 0.09× | 1.25× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.070 ms**; native kernel **0.006 ms**; TA-Lib 0.035 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.348 | 0.190 | 5.26M | 33.230 | 174.78× | 154.59× |
| 1,500 | 10 | 1.820 | 0.749 | 13.35M | 30.755 | 41.04× | 40.13× |
| 1,500 | 100 | 10.327 | 2.674 | 37.40M | 31.919 | 11.94× | 11.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
