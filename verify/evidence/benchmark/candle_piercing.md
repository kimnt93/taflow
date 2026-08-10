# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 54.73M | 0.015 | 66.84M | 0.033 | 1.82× | 2.22× |
| 10,000 | 0.128 | 78.00M | 0.125 | 80.12M | 0.127 | 0.99× | 1.02× |
| 100,000 | 1.290 | 77.49M | 1.329 | 75.24M | 1.049 | 0.81× | 0.79× |
| 1,000,000 | 13.166 | 75.95M | 12.885 | 77.61M | 9.953 | 0.76× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.092 | 0.141 | 1.54× |
| 1 | 5 | 0.344 | 0.443 | 1.29× |
| 1 | 10 | 0.603 | 1.042 | 1.73× |
| 10 | 1 | 0.066 | 0.110 | 1.67× |
| 10 | 5 | 0.243 | 0.434 | 1.78× |
| 10 | 10 | 0.544 | 0.962 | 1.77× |
| 100 | 1 | 0.065 | 0.088 | 1.36× |
| 100 | 5 | 0.254 | 0.442 | 1.74× |
| 100 | 10 | 0.566 | 0.981 | 1.73× |
| 1,000 | 1 | 0.079 | 0.129 | 1.63× |
| 1,000 | 5 | 0.297 | 0.553 | 1.86× |
| 1,000 | 10 | 0.623 | 1.066 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
