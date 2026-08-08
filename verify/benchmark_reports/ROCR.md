# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.08M | 0.004 | 236.48M | 0.032 | 0.71× | 7.56× |
| 10,000 | 0.460 | 21.73M | 0.036 | 279.69M | 0.042 | 0.09× | 1.18× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.069 ms**; native kernel **0.006 ms**; TA-Lib 0.032 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.288 | 0.179 | 5.59M | 31.510 | 176.06× | 169.56× |
| 1,500 | 10 | 1.871 | 2.193 | 4.56M | 31.969 | 14.57× | 14.16× |
| 1,500 | 100 | 6.712 | 2.264 | 44.17M | 44.329 | 19.58× | 13.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
