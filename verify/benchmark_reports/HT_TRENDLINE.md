# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.115 | 8.72M | 0.076 | 13.20M | 0.079 | 0.69× | 1.05× |
| 10,000 | 1.212 | 8.25M | 0.791 | 12.64M | 0.636 | 0.52× | 0.80× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.177 ms**; native kernel **0.115 ms**; TA-Lib 0.109 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.359 | 0.257 | 3.89M | 108.704 | 423.28× | 108.45× |
| 1,500 | 10 | 2.051 | 1.406 | 7.11M | 109.645 | 77.99× | 20.78× |
| 1,500 | 100 | 13.020 | 9.762 | 10.24M | 116.631 | 11.95× | 3.40× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
