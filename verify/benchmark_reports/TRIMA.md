# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 20.05M | 0.009 | 108.61M | 0.034 | 0.68× | 3.69× |
| 10,000 | 0.484 | 20.65M | 0.079 | 126.37M | 0.062 | 0.13× | 0.78× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.073 ms**; native kernel **0.013 ms**; TA-Lib 0.036 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.301 | 0.185 | 5.40M | 35.865 | 193.66× | 171.32× |
| 1,500 | 10 | 1.637 | 0.681 | 14.68M | 35.693 | 52.41× | 44.94× |
| 1,500 | 100 | 6.458 | 3.022 | 33.09M | 37.446 | 12.39× | 10.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
