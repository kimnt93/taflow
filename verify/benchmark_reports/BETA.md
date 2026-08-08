# RollingBeta benchmark (`BETA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.260 | 3.84M | 0.010 | 100.64M | 0.037 | 0.14× | 3.73× |
| 10,000 | 2.575 | 3.88M | 0.087 | 114.78M | 0.087 | 0.03× | 1.00× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.409 ms**; native kernel **0.014 ms**; TA-Lib 0.043 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.312 | 0.212 | 4.71M | 43.477 | 204.62× | 147.75× |
| 1,500 | 10 | 4.669 | 0.939 | 10.65M | 40.265 | 42.89× | 34.20× |
| 1,500 | 100 | 28.922 | 3.120 | 32.05M | 40.842 | 13.09× | 10.05× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
