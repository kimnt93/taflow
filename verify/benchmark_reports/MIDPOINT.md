# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.064 | 15.65M | 0.018 | 55.53M | 0.036 | 0.56× | 2.00× |
| 10,000 | 0.652 | 15.34M | 0.256 | 39.02M | 0.102 | 0.16× | 0.40× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.094 ms**; native kernel **0.030 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.355 | 0.203 | 4.92M | 40.577 | 199.75× | 160.65× |
| 1,500 | 10 | 1.992 | 1.000 | 10.00M | 37.360 | 37.36× | 30.39× |
| 1,500 | 100 | 10.520 | 24.159 | 4.14M | 39.526 | 1.64× | 1.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
