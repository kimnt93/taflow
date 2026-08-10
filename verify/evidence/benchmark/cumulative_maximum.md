# CumulativeMaximum benchmark (`numpy.maximum.accumulate` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 174.72M | 0.005 | 212.42M | 0.016 | 2.73× | 3.31× |
| 10,000 | 0.031 | 327.50M | 0.028 | 356.40M | 0.039 | 1.28× | 1.39× |
| 100,000 | 0.289 | 345.89M | 0.263 | 380.34M | 0.271 | 0.94× | 1.03× |
| 1,000,000 | 3.195 | 313.03M | 2.972 | 336.47M | 2.630 | 0.82× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.112 | 0.092 | 0.82× |
| 1 | 5 | 0.280 | 0.288 | 1.03× |
| 1 | 10 | 0.453 | 0.590 | 1.30× |
| 10 | 1 | 0.045 | 0.059 | 1.29× |
| 10 | 5 | 0.216 | 0.281 | 1.30× |
| 10 | 10 | 0.464 | 0.608 | 1.31× |
| 100 | 1 | 0.045 | 0.076 | 1.68× |
| 100 | 5 | 0.219 | 0.267 | 1.22× |
| 100 | 10 | 0.443 | 0.567 | 1.28× |
| 1,000 | 1 | 0.049 | 0.060 | 1.22× |
| 1,000 | 5 | 0.218 | 0.318 | 1.46× |
| 1,000 | 10 | 0.462 | 0.738 | 1.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
