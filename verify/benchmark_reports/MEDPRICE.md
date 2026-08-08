# MedianPrice benchmark (`MEDPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.75M | 0.001 | 818.36M | 0.029 | 0.64× | 23.96× |
| 10,000 | 0.429 | 23.32M | 0.005 | 2.15G | 0.033 | 0.08× | 7.15× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.066 ms**; native kernel **0.001 ms**; TA-Lib 0.028 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.344 | 0.216 | 4.63M | 29.782 | 137.91× | 125.15× |
| 1,500 | 10 | 2.064 | 0.856 | 11.68M | 28.192 | 32.92× | 31.87× |
| 1,500 | 100 | 8.757 | 2.231 | 44.82M | 28.488 | 12.77× | 12.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
