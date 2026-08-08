# MathCosh benchmark (`COSH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 20.17M | 0.007 | 133.65M | 0.035 | 0.71× | 4.72× |
| 10,000 | 0.475 | 21.06M | 0.066 | 151.30M | 0.087 | 0.18× | 1.31× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.071 ms**; native kernel **0.011 ms**; TA-Lib 0.038 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.340 | 0.188 | 5.32M | 37.059 | 197.03× | 140.40× |
| 1,500 | 10 | 1.796 | 0.694 | 14.40M | 36.571 | 52.67× | 40.22× |
| 1,500 | 100 | 7.458 | 2.734 | 36.58M | 39.316 | 14.38× | 10.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
