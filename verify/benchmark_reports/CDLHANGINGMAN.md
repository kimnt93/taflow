# CandleHangingMan benchmark (`CDLHANGINGMAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.032 | 30.90M | 0.033 | 30.65M | 0.040 | 1.23× | 1.22× |
| 10,000 | 0.313 | 31.97M | 0.324 | 30.87M | 0.175 | 0.56× | 0.54× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.048 ms**; native kernel **0.047 ms**; TA-Lib 0.048 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.701 | 0.323 | 3.09M | 48.706 | 150.69× | 87.72× |
| 1,500 | 10 | 2.877 | 1.449 | 6.90M | 49.308 | 34.04× | 20.06× |
| 1,500 | 100 | 8.378 | 5.720 | 17.48M | 54.420 | 9.51× | 5.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
