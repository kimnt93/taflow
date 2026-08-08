# MovingAverage benchmark (`MA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.24M | 0.006 | 156.64M | 0.035 | 0.75× | 5.56× |
| 10,000 | 0.443 | 22.59M | 0.054 | 183.77M | 0.055 | 0.12× | 1.01× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.068 ms**; native kernel **0.009 ms**; TA-Lib 0.039 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.325 | 0.177 | 5.65M | 36.496 | 206.08× | 181.67× |
| 1,500 | 10 | 1.336 | 0.715 | 13.99M | 36.012 | 50.39× | 45.97× |
| 1,500 | 100 | 5.712 | 2.524 | 39.62M | 36.863 | 14.60× | 13.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
