# HilbertTransformTrendMode benchmark (`HT_TRENDMODE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.161 | 6.23M | 0.167 | 5.97M | 0.473 | 2.95× | 2.83× |
| 10,000 | 1.650 | 6.06M | 1.723 | 5.80M | 4.384 | 2.66× | 2.54× |
| 100,000 | 16.736 | 5.97M | 16.553 | 6.04M | 45.381 | 2.71× | 2.74× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.106 | 1.12× |
| 1 | 5 | 0.376 | 0.473 | 1.26× |
| 1 | 10 | 0.502 | 0.884 | 1.76× |
| 10 | 1 | 0.048 | 0.085 | 1.75× |
| 10 | 5 | 0.229 | 0.399 | 1.74× |
| 10 | 10 | 0.461 | 0.864 | 1.88× |
| 100 | 1 | 0.065 | 0.118 | 1.81× |
| 100 | 5 | 0.231 | 0.553 | 2.39× |
| 100 | 10 | 0.493 | 1.189 | 2.41× |
| 1,000 | 1 | 0.235 | 0.568 | 2.42× |
| 1,000 | 5 | 0.398 | 2.868 | 7.21× |
| 1,000 | 10 | 0.631 | 5.591 | 8.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
