# RollingMedian benchmark (`MedianMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 25.06M | 0.036 | 27.46M | 0.341 | 8.54× | 9.36× |
| 10,000 | 0.399 | 25.09M | 0.409 | 24.45M | 1.884 | 4.73× | 4.61× |
| 100,000 | 4.186 | 23.89M | 4.031 | 24.81M | 19.860 | 4.74× | 4.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.269 | 3.23× |
| 1 | 5 | 0.258 | 1.068 | 4.14× |
| 1 | 10 | 0.507 | 2.654 | 5.24× |
| 10 | 1 | 0.052 | 0.209 | 4.04× |
| 10 | 5 | 0.235 | 1.117 | 4.74× |
| 10 | 10 | 0.516 | 2.470 | 4.78× |
| 100 | 1 | 0.058 | 0.233 | 4.00× |
| 100 | 5 | 0.267 | 1.464 | 5.49× |
| 100 | 10 | 0.596 | 2.604 | 4.37× |
| 1,000 | 1 | 0.115 | 0.443 | 3.84× |
| 1,000 | 5 | 0.260 | 2.160 | 8.31× |
| 1,000 | 10 | 0.568 | 4.164 | 7.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
