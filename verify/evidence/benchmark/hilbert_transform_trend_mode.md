# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.188 | 5.31M | 0.185 | 5.40M | 0.537 | 2.85× | 2.90× |
| 10,000 | 1.896 | 5.27M | 2.420 | 4.13M | 5.242 | 2.76× | 2.17× |
| 100,000 | 19.442 | 5.14M | 21.305 | 4.69M | 58.174 | 2.99× | 2.73× |
| 1,000,000 | 197.797 | 5.06M | 188.927 | 5.29M | 546.734 | 2.76× | 2.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.162 | 1.71× |
| 1 | 5 | 0.249 | 0.499 | 2.00× |
| 1 | 10 | 0.488 | 1.023 | 2.09× |
| 10 | 1 | 0.055 | 0.084 | 1.52× |
| 10 | 5 | 0.289 | 0.445 | 1.54× |
| 10 | 10 | 0.479 | 0.998 | 2.09× |
| 100 | 1 | 0.069 | 0.126 | 1.83× |
| 100 | 5 | 0.250 | 0.647 | 2.59× |
| 100 | 10 | 0.565 | 1.264 | 2.24× |
| 1,000 | 1 | 0.258 | 0.627 | 2.43× |
| 1,000 | 5 | 0.435 | 2.849 | 6.55× |
| 1,000 | 10 | 0.832 | 6.517 | 7.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
