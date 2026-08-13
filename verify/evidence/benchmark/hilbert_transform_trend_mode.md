# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.871 | 1.15M | 0.890 | 1.12M | 0.495 | 0.57× | 0.56× |
| 10,000 | 8.893 | 1.12M | 8.857 | 1.13M | 4.507 | 0.51× | 0.51× |
| 100,000 | 89.876 | 1.11M | 92.438 | 1.08M | 45.165 | 0.50× | 0.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.116 | 1.05× |
| 1 | 5 | 0.554 | 0.473 | 0.85× |
| 1 | 10 | 0.641 | 0.893 | 1.39× |
| 10 | 1 | 0.072 | 0.089 | 1.24× |
| 10 | 5 | 0.298 | 0.406 | 1.36× |
| 10 | 10 | 0.669 | 0.898 | 1.34× |
| 100 | 1 | 0.127 | 0.123 | 0.97× |
| 100 | 5 | 0.320 | 0.558 | 1.74× |
| 100 | 10 | 0.710 | 1.165 | 1.64× |
| 1,000 | 1 | 0.991 | 0.558 | 0.56× |
| 1,000 | 5 | 1.167 | 2.801 | 2.40× |
| 1,000 | 10 | 2.037 | 5.590 | 2.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
