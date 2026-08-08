# RollingLinearRegression benchmark (`LINEARREG` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.100 | 10.04M | 0.030 | 33.69M | 0.049 | 0.49× | 1.66× |
| 10,000 | 0.712 | 14.05M | 0.271 | 36.96M | 0.168 | 0.24× | 0.62× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.107 ms**; native kernel **0.041 ms**; TA-Lib 0.053 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.311 | 0.206 | 4.85M | 50.847 | 246.37× | 162.24× |
| 1,500 | 10 | 3.565 | 1.041 | 9.61M | 50.579 | 48.59× | 30.61× |
| 1,500 | 100 | 10.706 | 4.665 | 21.44M | 56.374 | 12.08× | 7.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
