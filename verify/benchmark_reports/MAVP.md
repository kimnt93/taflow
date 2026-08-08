# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.257 | 3.89M | 0.209 | 4.79M | 0.117 | 0.45× | 0.56× |
| 10,000 | 2.639 | 3.79M | 2.136 | 4.68M | 0.786 | 0.30× | 0.37× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.366 ms**; native kernel **0.313 ms**; TA-Lib 0.154 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.625 | 0.417 | 2.40M | 161.377 | 386.64× | 91.65× |
| 1,500 | 10 | 4.554 | 2.938 | 3.40M | 156.094 | 53.13× | 13.02× |
| 1,500 | 100 | 28.480 | 24.132 | 4.14M | 164.558 | 6.82× | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
