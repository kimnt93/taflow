# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.218 | 4.58M | 0.055 | 18.02M | 0.055 | 0.25× | 0.99× |
| 10,000 | 2.148 | 4.66M | 0.592 | 16.89M | 0.210 | 0.10× | 0.35× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.310 ms**; native kernel **0.087 ms**; TA-Lib 0.067 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.327 | 0.259 | 3.87M | 60.254 | 233.05× | 176.39× |
| 1,500 | 10 | 3.882 | 1.336 | 7.49M | 63.080 | 47.23× | 32.83× |
| 1,500 | 100 | 19.775 | 7.819 | 12.79M | 68.965 | 8.82× | 9.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
