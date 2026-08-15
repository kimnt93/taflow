# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 203.74M | 0.004 | 241.04M | 0.030 | 6.08× | 7.20× |
| 10,000 | 0.035 | 287.95M | 0.032 | 317.19M | 0.046 | 1.33× | 1.46× |
| 100,000 | 0.337 | 296.58M | 0.313 | 319.05M | 0.205 | 0.61× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.101 | 1.26× |
| 1 | 5 | 0.333 | 0.475 | 1.43× |
| 1 | 10 | 0.366 | 0.892 | 2.44× |
| 10 | 1 | 0.039 | 0.086 | 2.20× |
| 10 | 5 | 0.181 | 0.420 | 2.31× |
| 10 | 10 | 0.384 | 0.897 | 2.34× |
| 100 | 1 | 0.043 | 0.084 | 1.97× |
| 100 | 5 | 0.210 | 0.457 | 2.18× |
| 100 | 10 | 0.389 | 0.877 | 2.26× |
| 1,000 | 1 | 0.049 | 0.089 | 1.82× |
| 1,000 | 5 | 0.206 | 0.446 | 2.16× |
| 1,000 | 10 | 0.430 | 0.899 | 2.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
