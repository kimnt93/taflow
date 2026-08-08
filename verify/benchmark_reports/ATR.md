# AverageTrueRange benchmark (`ATR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.45M | 0.006 | 157.06M | 0.040 | 0.81× | 6.23× |
| 10,000 | 0.472 | 21.20M | 0.057 | 176.87M | 0.089 | 0.19× | 1.57× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.070 ms**; native kernel **0.009 ms**; TA-Lib 0.040 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.607 | 0.253 | 3.95M | 42.957 | 169.61× | 128.50× |
| 1,500 | 10 | 2.754 | 1.083 | 9.23M | 41.617 | 38.43× | 28.16× |
| 1,500 | 100 | 7.829 | 2.966 | 33.72M | 42.234 | 14.24× | 10.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
