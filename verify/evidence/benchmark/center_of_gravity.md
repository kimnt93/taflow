# CenterOfGravity benchmark (`CenterOfGravity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.93M | 0.020 | 50.43M | 0.168 | 8.40× | 8.49× |
| 10,000 | 0.188 | 53.32M | 0.174 | 57.51M | 0.590 | 3.15× | 3.39× |
| 100,000 | 1.653 | 60.48M | 1.613 | 62.01M | 4.981 | 3.01× | 3.09× |
| 1,000,000 | 17.602 | 56.81M | 16.227 | 61.63M | 46.954 | 2.67× | 2.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.156 | 0.245 | 1.58× |
| 1 | 5 | 0.297 | 0.950 | 3.20× |
| 1 | 10 | 0.469 | 2.043 | 4.36× |
| 10 | 1 | 0.054 | 0.185 | 3.45× |
| 10 | 5 | 0.220 | 0.923 | 4.20× |
| 10 | 10 | 0.476 | 2.173 | 4.56× |
| 100 | 1 | 0.060 | 0.204 | 3.38× |
| 100 | 5 | 0.225 | 0.957 | 4.26× |
| 100 | 10 | 0.478 | 2.132 | 4.46× |
| 1,000 | 1 | 0.070 | 0.241 | 3.46× |
| 1,000 | 5 | 0.240 | 1.185 | 4.94× |
| 1,000 | 10 | 0.508 | 2.642 | 5.21× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
