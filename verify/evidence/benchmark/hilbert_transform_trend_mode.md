# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.259 | 3.85M | 0.178 | 5.60M | 0.522 | 2.01× | 2.92× |
| 10,000 | 1.760 | 5.68M | 1.783 | 5.61M | 4.829 | 2.74× | 2.71× |
| 100,000 | 17.870 | 5.60M | 18.374 | 5.44M | 66.711 | 3.73× | 3.63× |
| 1,000,000 | 190.218 | 5.26M | 187.242 | 5.34M | 499.958 | 2.63× | 2.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.147 | 1.37× |
| 1 | 5 | 0.318 | 0.534 | 1.68× |
| 1 | 10 | 0.495 | 0.873 | 1.76× |
| 10 | 1 | 0.045 | 0.082 | 1.84× |
| 10 | 5 | 0.217 | 0.431 | 1.99× |
| 10 | 10 | 0.515 | 0.889 | 1.73× |
| 100 | 1 | 0.064 | 0.123 | 1.93× |
| 100 | 5 | 0.242 | 0.582 | 2.40× |
| 100 | 10 | 0.507 | 1.301 | 2.56× |
| 1,000 | 1 | 0.239 | 0.592 | 2.48× |
| 1,000 | 5 | 0.406 | 3.086 | 7.60× |
| 1,000 | 10 | 0.613 | 5.919 | 9.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
