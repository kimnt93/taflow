# JurikMovingAverage benchmark (`jma` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.65M | 0.084 | 11.88M | 19.670 | 229.08× | 233.67× |
| 10,000 | 0.880 | 11.36M | 0.923 | 10.84M | 206.283 | 234.40× | 223.51× |
| 100,000 | 8.412 | 11.89M | 8.411 | 11.89M | 1969.943 | 234.19× | 234.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.225 | 1.25× |
| 1 | 5 | 0.300 | 0.835 | 2.78× |
| 1 | 10 | 0.417 | 1.817 | 4.36× |
| 10 | 1 | 0.046 | 0.505 | 10.91× |
| 10 | 5 | 0.193 | 2.575 | 13.37× |
| 10 | 10 | 0.413 | 4.745 | 11.48× |
| 100 | 1 | 0.053 | 2.298 | 43.31× |
| 100 | 5 | 0.215 | 11.676 | 54.25× |
| 100 | 10 | 0.525 | 24.025 | 45.76× |
| 1,000 | 1 | 0.180 | 20.529 | 114.35× |
| 1,000 | 5 | 0.432 | 110.097 | 255.00× |
| 1,000 | 10 | 0.607 | 255.348 | 420.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
