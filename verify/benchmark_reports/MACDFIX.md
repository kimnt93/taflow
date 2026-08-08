# MovingAverageConvergenceDivergenceFixed benchmark (`MACDFIX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.213 | 4.69M | 0.015 | 66.09M | 0.047 | 0.22× | 3.11× |
| 10,000 | 2.135 | 4.68M | 0.141 | 70.98M | 0.136 | 0.06× | 0.96× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.323 ms**; native kernel **0.023 ms**; TA-Lib 0.053 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.318 | 0.216 | 4.63M | 51.215 | 236.90× | 183.74× |
| 1,500 | 10 | 3.860 | 0.985 | 10.15M | 53.388 | 54.18× | 41.13× |
| 1,500 | 100 | 17.951 | 3.813 | 26.23M | 56.502 | 14.82× | 10.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
