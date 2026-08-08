# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.029 | 34.23M | 0.027 | 36.91M | 0.031 | 1.07× | 1.16× |
| 10,000 | 0.300 | 33.31M | 0.332 | 30.16M | 0.083 | 0.28× | 0.25× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.044 ms**; native kernel **0.040 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.389 | 0.307 | 3.26M | 32.517 | 106.02× | 94.06× |
| 1,500 | 10 | 2.876 | 1.442 | 6.94M | 33.024 | 22.90× | 20.89× |
| 1,500 | 100 | 12.277 | 8.177 | 12.23M | 33.809 | 4.13× | 3.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
