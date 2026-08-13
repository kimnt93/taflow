# RollingMinimumIndex benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.051 | 19.71M | 0.048 | 20.85M | 0.034 | 0.68× | 0.71× |
| 10,000 | 0.521 | 19.19M | 0.487 | 20.54M | 0.091 | 0.17× | 0.19× |
| 100,000 | 5.043 | 19.83M | 4.742 | 21.09M | 0.676 | 0.13× | 0.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.165 | 0.136 | 0.82× |
| 1 | 5 | 0.367 | 0.444 | 1.21× |
| 1 | 10 | 0.609 | 0.921 | 1.51× |
| 10 | 1 | 0.064 | 0.086 | 1.34× |
| 10 | 5 | 0.308 | 0.430 | 1.40× |
| 10 | 10 | 0.582 | 0.900 | 1.55× |
| 100 | 1 | 0.075 | 0.091 | 1.22× |
| 100 | 5 | 0.309 | 0.423 | 1.37× |
| 100 | 10 | 0.634 | 0.892 | 1.41× |
| 1,000 | 1 | 0.104 | 0.093 | 0.90× |
| 1,000 | 5 | 0.299 | 0.478 | 1.60× |
| 1,000 | 10 | 0.612 | 0.978 | 1.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
