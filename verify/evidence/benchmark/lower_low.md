# LowerLow benchmark (`lower low relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 402.70M | 0.001 | 872.58M | 0.021 | 8.58× | 18.59× |
| 10,000 | 0.008 | 1.25G | 0.005 | 2.05G | 0.039 | 4.90× | 8.01× |
| 100,000 | 0.074 | 1.34G | 0.052 | 1.91G | 0.232 | 3.11× | 4.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.103 | 0.081 | 0.78× |
| 1 | 5 | 0.269 | 0.350 | 1.30× |
| 1 | 10 | 0.369 | 0.715 | 1.94× |
| 10 | 1 | 0.041 | 0.071 | 1.76× |
| 10 | 5 | 0.178 | 0.327 | 1.83× |
| 10 | 10 | 0.401 | 0.707 | 1.76× |
| 100 | 1 | 0.040 | 0.069 | 1.73× |
| 100 | 5 | 0.181 | 0.337 | 1.86× |
| 100 | 10 | 0.364 | 0.719 | 1.97× |
| 1,000 | 1 | 0.041 | 0.075 | 1.85× |
| 1,000 | 5 | 0.174 | 0.504 | 2.90× |
| 1,000 | 10 | 0.353 | 1.107 | 3.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
