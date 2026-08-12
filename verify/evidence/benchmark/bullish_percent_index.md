# BullishPercentIndex benchmark (`BullishPercentIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.17M | 0.005 | 184.67M | 11.634 | 1607.48× | 2148.48× |
| 10,000 | 0.031 | 319.80M | 0.027 | 368.74M | 121.570 | 3887.83× | 4482.76× |
| 100,000 | 0.249 | 400.94M | 0.285 | 350.87M | 1183.700 | 4745.89× | 4153.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.303 | 3.18× |
| 1 | 5 | 0.308 | 1.489 | 4.83× |
| 1 | 10 | 0.520 | 2.433 | 4.67× |
| 10 | 1 | 0.051 | 0.331 | 6.48× |
| 10 | 5 | 0.251 | 1.736 | 6.90× |
| 10 | 10 | 0.542 | 3.849 | 7.10× |
| 100 | 1 | 0.056 | 1.461 | 25.94× |
| 100 | 5 | 0.314 | 7.828 | 24.91× |
| 100 | 10 | 0.553 | 15.464 | 27.97× |
| 1,000 | 1 | 0.051 | 12.241 | 240.04× |
| 1,000 | 5 | 0.529 | 65.398 | 123.67× |
| 1,000 | 10 | 0.627 | 126.094 | 201.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
