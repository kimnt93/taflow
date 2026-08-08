# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 74.97M | 0.011 | 88.52M | 0.035 | 2.60× | 3.08× |
| 10,000 | 0.137 | 72.91M | 0.134 | 74.71M | 0.140 | 1.02× | 1.04× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.020 ms**; native kernel **0.020 ms**; TA-Lib 0.041 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.375 | 0.291 | 3.43M | 39.899 | 136.92× | 94.92× |
| 1,500 | 10 | 2.691 | 1.239 | 8.07M | 38.104 | 30.74× | 23.61× |
| 1,500 | 100 | 6.582 | 4.040 | 24.75M | 39.791 | 9.85× | 7.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
