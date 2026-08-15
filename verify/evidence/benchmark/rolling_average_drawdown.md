# RollingAverageDrawdown benchmark (`AverageDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.90M | 0.046 | 21.81M | 0.226 | 4.95× | 4.93× |
| 10,000 | 0.495 | 20.20M | 0.489 | 20.47M | 1.000 | 2.02× | 2.05× |
| 100,000 | 4.922 | 20.32M | 4.850 | 20.62M | 9.499 | 1.93× | 1.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.239 | 3.52× |
| 1 | 5 | 0.257 | 1.013 | 3.94× |
| 1 | 10 | 0.430 | 2.146 | 4.99× |
| 10 | 1 | 0.043 | 0.196 | 4.58× |
| 10 | 5 | 0.193 | 0.976 | 5.07× |
| 10 | 10 | 0.414 | 2.148 | 5.19× |
| 100 | 1 | 0.048 | 0.206 | 4.27× |
| 100 | 5 | 0.190 | 1.057 | 5.57× |
| 100 | 10 | 0.451 | 2.198 | 4.87× |
| 1,000 | 1 | 0.105 | 0.321 | 3.07× |
| 1,000 | 5 | 0.229 | 1.490 | 6.49× |
| 1,000 | 10 | 0.461 | 3.114 | 6.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
