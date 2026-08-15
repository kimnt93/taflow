# FibonacciChannel benchmark (`FibChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 53.75M | 0.016 | 62.44M | 0.523 | 28.14× | 32.68× |
| 10,000 | 0.162 | 61.79M | 0.149 | 67.10M | 4.149 | 25.63× | 27.84× |
| 100,000 | 1.640 | 60.98M | 1.489 | 67.15M | 49.269 | 30.04× | 33.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.221 | 2.27× |
| 1 | 5 | 0.288 | 0.864 | 2.99× |
| 1 | 10 | 0.453 | 1.911 | 4.22× |
| 10 | 1 | 0.051 | 0.170 | 3.35× |
| 10 | 5 | 0.188 | 0.868 | 4.61× |
| 10 | 10 | 0.443 | 1.920 | 4.33× |
| 100 | 1 | 0.046 | 0.210 | 4.51× |
| 100 | 5 | 0.192 | 1.115 | 5.81× |
| 100 | 10 | 0.438 | 2.314 | 5.28× |
| 1,000 | 1 | 0.065 | 0.829 | 12.81× |
| 1,000 | 5 | 0.201 | 3.366 | 16.71× |
| 1,000 | 10 | 0.486 | 6.870 | 14.13× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
