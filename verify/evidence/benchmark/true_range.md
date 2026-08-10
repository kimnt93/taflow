# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.20M | 0.007 | 146.75M | 0.027 | 3.03× | 3.93× |
| 10,000 | 0.040 | 251.20M | 0.036 | 277.77M | 0.033 | 0.83× | 0.92× |
| 100,000 | 0.319 | 313.50M | 0.300 | 333.32M | 0.086 | 0.27× | 0.29× |
| 1,000,000 | 4.205 | 237.80M | 3.498 | 285.89M | 1.279 | 0.30× | 0.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.118 | 1.19× |
| 1 | 5 | 0.288 | 0.463 | 1.61× |
| 1 | 10 | 0.501 | 0.878 | 1.75× |
| 10 | 1 | 0.049 | 0.086 | 1.75× |
| 10 | 5 | 0.219 | 0.406 | 1.85× |
| 10 | 10 | 0.485 | 0.874 | 1.80× |
| 100 | 1 | 0.049 | 0.086 | 1.75× |
| 100 | 5 | 0.224 | 0.403 | 1.80× |
| 100 | 10 | 0.494 | 0.880 | 1.78× |
| 1,000 | 1 | 0.058 | 0.080 | 1.39× |
| 1,000 | 5 | 0.251 | 0.401 | 1.60× |
| 1,000 | 10 | 0.488 | 0.861 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
