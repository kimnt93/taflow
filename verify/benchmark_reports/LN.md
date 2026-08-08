# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.92M | 0.006 | 178.91M | 0.034 | 0.70× | 6.01× |
| 10,000 | 0.450 | 22.23M | 0.048 | 206.78M | 0.074 | 0.17× | 1.54× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.072 ms**; native kernel **0.009 ms**; TA-Lib 0.033 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.327 | 0.177 | 5.65M | 33.306 | 188.06× | 145.95× |
| 1,500 | 10 | 1.669 | 0.671 | 14.91M | 36.378 | 54.24× | 37.81× |
| 1,500 | 100 | 7.107 | 2.830 | 35.34M | 33.307 | 11.77× | 9.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
