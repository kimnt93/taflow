# RollingKellyCriterion benchmark (`KellyCriterion` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 51.07M | 0.018 | 55.46M | 0.189 | 9.66× | 10.49× |
| 10,000 | 0.182 | 54.89M | 0.180 | 55.71M | 0.690 | 3.79× | 3.84× |
| 100,000 | 1.676 | 59.68M | 1.655 | 60.41M | 5.425 | 3.24× | 3.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.221 | 2.18× |
| 1 | 5 | 0.322 | 0.938 | 2.92× |
| 1 | 10 | 0.402 | 2.121 | 5.27× |
| 10 | 1 | 0.056 | 0.199 | 3.52× |
| 10 | 5 | 0.192 | 0.924 | 4.81× |
| 10 | 10 | 0.417 | 2.167 | 5.20× |
| 100 | 1 | 0.048 | 0.193 | 4.03× |
| 100 | 5 | 0.192 | 0.969 | 5.05× |
| 100 | 10 | 0.435 | 2.288 | 5.26× |
| 1,000 | 1 | 0.066 | 0.257 | 3.89× |
| 1,000 | 5 | 0.225 | 1.269 | 5.64× |
| 1,000 | 10 | 0.410 | 2.723 | 6.65× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
