# AverageDirectionalIndexRating benchmark (`ADXR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.060 | 16.68M | 0.017 | 58.67M | 0.041 | 0.69× | 2.42× |
| 10,000 | 0.551 | 18.14M | 0.155 | 64.40M | 0.125 | 0.23× | 0.81× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.086 ms**; native kernel **0.024 ms**; TA-Lib 0.045 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.383 | 0.258 | 3.87M | 44.444 | 172.01× | 120.09× |
| 1,500 | 10 | 1.874 | 1.196 | 8.36M | 45.322 | 37.88× | 27.60× |
| 1,500 | 100 | 9.011 | 4.405 | 22.70M | 44.833 | 10.18× | 7.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
