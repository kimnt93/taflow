# RollingRank benchmark (`rolling percentile rank` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.71M | 0.020 | 49.09M | 0.155 | 8.33× | 7.62× |
| 10,000 | 0.177 | 56.39M | 0.170 | 58.72M | 0.739 | 4.16× | 4.34× |
| 100,000 | 1.652 | 60.53M | 1.755 | 56.97M | 7.250 | 4.39× | 4.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.072 | 0.130 | 1.81× |
| 1 | 5 | 0.235 | 0.538 | 2.29× |
| 1 | 10 | 0.403 | 1.289 | 3.20× |
| 10 | 1 | 0.057 | 0.118 | 2.09× |
| 10 | 5 | 0.244 | 0.542 | 2.22× |
| 10 | 10 | 0.391 | 1.094 | 2.80× |
| 100 | 1 | 0.050 | 0.167 | 3.31× |
| 100 | 5 | 0.231 | 0.896 | 3.88× |
| 100 | 10 | 0.432 | 1.622 | 3.76× |
| 1,000 | 1 | 0.059 | 0.221 | 3.73× |
| 1,000 | 5 | 0.229 | 1.060 | 4.62× |
| 1,000 | 10 | 0.456 | 2.167 | 4.75× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
