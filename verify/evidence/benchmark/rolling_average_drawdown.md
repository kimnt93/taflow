# RollingAverageDrawdown benchmark (`AverageDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.053 | 19.01M | 0.053 | 18.98M | 0.226 | 4.30× | 4.30× |
| 10,000 | 0.506 | 19.75M | 0.503 | 19.87M | 1.075 | 2.12× | 2.14× |
| 100,000 | 5.344 | 18.71M | 5.174 | 19.33M | 9.471 | 1.77× | 1.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.237 | 3.03× |
| 1 | 5 | 0.269 | 1.015 | 3.77× |
| 1 | 10 | 0.494 | 2.139 | 4.33× |
| 10 | 1 | 0.052 | 0.198 | 3.79× |
| 10 | 5 | 0.225 | 1.000 | 4.44× |
| 10 | 10 | 0.480 | 2.212 | 4.61× |
| 100 | 1 | 0.061 | 0.210 | 3.46× |
| 100 | 5 | 0.295 | 1.058 | 3.58× |
| 100 | 10 | 0.532 | 2.245 | 4.22× |
| 1,000 | 1 | 0.105 | 0.289 | 2.76× |
| 1,000 | 5 | 0.259 | 1.482 | 5.71× |
| 1,000 | 10 | 0.578 | 3.229 | 5.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
