# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.166 | 6.04M | 0.169 | 5.92M | 0.480 | 2.90× | 2.84× |
| 10,000 | 1.715 | 5.83M | 1.723 | 5.80M | 4.710 | 2.75× | 2.73× |
| 100,000 | 17.027 | 5.87M | 17.114 | 5.84M | 47.535 | 2.79× | 2.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.070 | 0.114 | 1.62× |
| 1 | 5 | 0.269 | 0.435 | 1.61× |
| 1 | 10 | 0.370 | 0.910 | 2.46× |
| 10 | 1 | 0.042 | 0.092 | 2.18× |
| 10 | 5 | 0.210 | 0.458 | 2.19× |
| 10 | 10 | 0.387 | 0.887 | 2.29× |
| 100 | 1 | 0.057 | 0.119 | 2.09× |
| 100 | 5 | 0.196 | 0.562 | 2.88× |
| 100 | 10 | 0.435 | 1.194 | 2.74× |
| 1,000 | 1 | 0.229 | 0.563 | 2.46× |
| 1,000 | 5 | 0.335 | 2.855 | 8.51× |
| 1,000 | 10 | 0.577 | 5.661 | 9.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
