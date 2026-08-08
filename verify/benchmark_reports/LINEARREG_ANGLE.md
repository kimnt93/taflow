# RollingLinearRegressionAngle benchmark (`LINEARREG_ANGLE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.082 | 12.21M | 0.039 | 25.39M | 0.056 | 0.69× | 1.43× |
| 10,000 | 0.800 | 12.51M | 0.372 | 26.87M | 0.256 | 0.32× | 0.69× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.120 ms**; native kernel **0.055 ms**; TA-Lib 0.067 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.343 | 0.386 | 2.59M | 61.104 | 158.12× | 74.84× |
| 1,500 | 10 | 2.262 | 1.501 | 6.66M | 63.980 | 42.63× | 21.42× |
| 1,500 | 100 | 12.353 | 6.110 | 16.37M | 69.828 | 11.43× | 5.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
