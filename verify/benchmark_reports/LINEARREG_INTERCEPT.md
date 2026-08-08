# RollingLinearRegressionIntercept benchmark (`LINEARREG_INTERCEPT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.070 | 14.28M | 0.026 | 38.17M | 0.043 | 0.62× | 1.65× |
| 10,000 | 0.724 | 13.82M | 0.255 | 39.24M | 0.165 | 0.23× | 0.65× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.108 ms**; native kernel **0.041 ms**; TA-Lib 0.051 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.315 | 0.203 | 4.94M | 50.647 | 250.07× | 149.66× |
| 1,500 | 10 | 2.072 | 0.977 | 10.23M | 52.714 | 53.94× | 31.86× |
| 1,500 | 100 | 9.435 | 4.614 | 21.67M | 53.783 | 11.66× | 6.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
