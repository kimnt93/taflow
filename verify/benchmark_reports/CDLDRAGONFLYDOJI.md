# CandleDragonflyDoji benchmark (`CDLDRAGONFLYDOJI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 118.11M | 0.007 | 151.21M | 0.033 | 3.90× | 5.00× |
| 10,000 | 0.084 | 118.60M | 0.077 | 130.52M | 0.100 | 1.19× | 1.31× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.012 ms**; native kernel **0.010 ms**; TA-Lib 0.037 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.590 | 0.289 | 3.46M | 37.137 | 128.54× | 97.13× |
| 1,500 | 10 | 2.672 | 1.204 | 8.31M | 36.851 | 30.62× | 23.13× |
| 1,500 | 100 | 6.634 | 3.448 | 29.00M | 50.525 | 14.65× | 12.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
