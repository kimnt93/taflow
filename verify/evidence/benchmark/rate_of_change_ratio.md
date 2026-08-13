# RateOfChangeRatio benchmark (`ROCR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.038 | 26.29M | 0.030 | 32.85M | 0.031 | 0.81× | 1.01× |
| 10,000 | 0.254 | 39.34M | 0.234 | 42.65M | 0.040 | 0.16× | 0.17× |
| 100,000 | 2.275 | 43.96M | 2.283 | 43.80M | 0.122 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.124 | 1.21× |
| 1 | 5 | 0.381 | 0.519 | 1.36× |
| 1 | 10 | 0.623 | 0.917 | 1.47× |
| 10 | 1 | 0.063 | 0.092 | 1.48× |
| 10 | 5 | 0.293 | 0.436 | 1.49× |
| 10 | 10 | 0.609 | 0.908 | 1.49× |
| 100 | 1 | 0.068 | 0.089 | 1.30× |
| 100 | 5 | 0.309 | 0.473 | 1.53× |
| 100 | 10 | 0.734 | 0.939 | 1.28× |
| 1,000 | 1 | 0.087 | 0.094 | 1.08× |
| 1,000 | 5 | 0.321 | 0.460 | 1.43× |
| 1,000 | 10 | 0.611 | 0.932 | 1.53× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
