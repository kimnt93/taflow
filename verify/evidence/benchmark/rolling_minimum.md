# RollingMinimum benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 173.24M | 0.005 | 196.72M | 0.038 | 6.51× | 7.39× |
| 10,000 | 0.038 | 265.57M | 0.035 | 285.60M | 0.082 | 2.17× | 2.33× |
| 100,000 | 0.379 | 263.58M | 0.354 | 282.33M | 0.543 | 1.43× | 1.53× |
| 1,000,000 | 4.357 | 229.50M | 3.810 | 262.46M | 5.229 | 1.20× | 1.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.171 | 1.51× |
| 1 | 5 | 0.353 | 0.471 | 1.34× |
| 1 | 10 | 0.467 | 0.995 | 2.13× |
| 10 | 1 | 0.056 | 0.093 | 1.66× |
| 10 | 5 | 0.207 | 0.462 | 2.23× |
| 10 | 10 | 0.458 | 0.923 | 2.01× |
| 100 | 1 | 0.047 | 0.090 | 1.94× |
| 100 | 5 | 0.219 | 0.482 | 2.20× |
| 100 | 10 | 0.486 | 1.027 | 2.11× |
| 1,000 | 1 | 0.051 | 0.105 | 2.05× |
| 1,000 | 5 | 0.229 | 0.485 | 2.12× |
| 1,000 | 10 | 0.540 | 1.013 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
