# HilbertTransformTrendline benchmark (`HT_TRENDLINE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.293 | 3.41M | 0.286 | 3.50M | 0.078 | 0.27× | 0.27× |
| 10,000 | 3.103 | 3.22M | 2.873 | 3.48M | 0.636 | 0.20× | 0.22× |
| 100,000 | 30.281 | 3.30M | 29.410 | 3.40M | 5.587 | 0.18× | 0.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.100 | 0.88× |
| 1 | 5 | 0.487 | 0.434 | 0.89× |
| 1 | 10 | 0.607 | 0.875 | 1.44× |
| 10 | 1 | 0.061 | 0.086 | 1.42× |
| 10 | 5 | 0.272 | 0.439 | 1.61× |
| 10 | 10 | 0.618 | 0.905 | 1.47× |
| 100 | 1 | 0.081 | 0.100 | 1.23× |
| 100 | 5 | 0.302 | 0.446 | 1.48× |
| 100 | 10 | 0.630 | 0.926 | 1.47× |
| 1,000 | 1 | 0.374 | 0.147 | 0.39× |
| 1,000 | 5 | 0.586 | 0.718 | 1.22× |
| 1,000 | 10 | 0.967 | 1.502 | 1.55× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
