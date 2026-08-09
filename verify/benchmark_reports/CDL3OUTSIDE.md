# CandleThreeOutside benchmark (`CDL3OUTSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 117.33M | 0.007 | 153.61M | 0.032 | 3.81× | 4.99× |
| 10,000 | 0.071 | 141.34M | 0.068 | 147.86M | 0.088 | 1.24× | 1.30× |
| 100,000 | 0.742 | 134.72M | 0.690 | 144.88M | 0.575 | 0.77× | 0.83× |
| 1,000,000 | 7.577 | 131.97M | 7.427 | 134.65M | 5.794 | 0.76× | 0.78× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.151 | 1.28× |
| 1 | 5 | 0.314 | 0.493 | 1.57× |
| 1 | 10 | 0.513 | 0.949 | 1.85× |
| 10 | 1 | 0.054 | 0.094 | 1.72× |
| 10 | 5 | 0.232 | 0.414 | 1.78× |
| 10 | 10 | 0.535 | 0.914 | 1.71× |
| 100 | 1 | 0.055 | 0.093 | 1.70× |
| 100 | 5 | 0.253 | 0.445 | 1.75× |
| 100 | 10 | 0.525 | 0.902 | 1.72× |
| 1,000 | 1 | 0.060 | 0.100 | 1.68× |
| 1,000 | 5 | 0.241 | 0.459 | 1.90× |
| 1,000 | 10 | 0.520 | 0.979 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
