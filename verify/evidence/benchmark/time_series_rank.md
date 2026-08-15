# TimeSeriesRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.53M | 0.019 | 53.37M | 0.139 | 7.57× | 7.41× |
| 10,000 | 0.168 | 59.64M | 0.164 | 60.95M | 0.751 | 4.48× | 4.58× |
| 100,000 | 1.647 | 60.72M | 1.639 | 61.02M | 6.848 | 4.16× | 4.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.055 | 0.131 | 2.39× |
| 1 | 5 | 0.213 | 0.545 | 2.56× |
| 1 | 10 | 0.455 | 1.113 | 2.45× |
| 10 | 1 | 0.043 | 0.111 | 2.56× |
| 10 | 5 | 0.178 | 0.522 | 2.93× |
| 10 | 10 | 0.403 | 1.171 | 2.90× |
| 100 | 1 | 0.052 | 0.161 | 3.10× |
| 100 | 5 | 0.201 | 0.768 | 3.82× |
| 100 | 10 | 0.431 | 1.758 | 4.08× |
| 1,000 | 1 | 0.064 | 0.217 | 3.41× |
| 1,000 | 5 | 0.210 | 0.960 | 4.56× |
| 1,000 | 10 | 0.434 | 2.166 | 4.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
