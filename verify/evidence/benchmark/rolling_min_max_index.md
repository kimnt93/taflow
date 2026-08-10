# RollingMinMaxIndex benchmark (`MINMAXINDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 42.61M | 0.023 | 43.01M | 0.044 | 1.86× | 1.88× |
| 10,000 | 0.286 | 34.93M | 0.286 | 34.95M | 0.159 | 0.56× | 0.56× |
| 100,000 | 2.756 | 36.28M | 2.724 | 36.72M | 1.197 | 0.43× | 0.44× |
| 1,000,000 | 28.869 | 34.64M | 27.326 | 36.60M | 12.100 | 0.42× | 0.44× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.143 | 1.29× |
| 1 | 5 | 0.302 | 0.659 | 2.18× |
| 1 | 10 | 0.546 | 0.986 | 1.81× |
| 10 | 1 | 0.054 | 0.092 | 1.70× |
| 10 | 5 | 0.223 | 0.444 | 1.99× |
| 10 | 10 | 0.482 | 0.965 | 2.00× |
| 100 | 1 | 0.051 | 0.104 | 2.04× |
| 100 | 5 | 0.239 | 0.471 | 1.97× |
| 100 | 10 | 0.492 | 0.986 | 2.00× |
| 1,000 | 1 | 0.083 | 0.114 | 1.37× |
| 1,000 | 5 | 0.254 | 0.573 | 2.25× |
| 1,000 | 10 | 0.511 | 1.120 | 2.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
