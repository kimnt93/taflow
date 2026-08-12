# TriangularMovingAverage benchmark (`TRIMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.08M | 0.007 | 147.72M | 0.038 | 4.55× | 5.55× |
| 10,000 | 0.045 | 221.32M | 0.043 | 230.59M | 0.061 | 1.35× | 1.41× |
| 100,000 | 0.425 | 235.53M | 0.426 | 234.83M | 0.329 | 0.77× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.172 | 0.105 | 0.61× |
| 1 | 5 | 0.326 | 0.465 | 1.43× |
| 1 | 10 | 0.439 | 0.935 | 2.13× |
| 10 | 1 | 0.047 | 0.088 | 1.88× |
| 10 | 5 | 0.252 | 0.499 | 1.98× |
| 10 | 10 | 0.458 | 0.934 | 2.04× |
| 100 | 1 | 0.048 | 0.104 | 2.16× |
| 100 | 5 | 0.220 | 0.441 | 2.01× |
| 100 | 10 | 0.513 | 0.985 | 1.92× |
| 1,000 | 1 | 0.055 | 0.097 | 1.77× |
| 1,000 | 5 | 0.238 | 0.452 | 1.90× |
| 1,000 | 10 | 0.487 | 1.081 | 2.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
