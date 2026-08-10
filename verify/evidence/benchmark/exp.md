# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 133.55M | 0.007 | 147.27M | 0.031 | 4.13× | 4.55× |
| 10,000 | 0.052 | 192.28M | 0.049 | 202.12M | 0.070 | 1.35× | 1.42× |
| 100,000 | 0.495 | 201.82M | 0.473 | 211.31M | 0.476 | 0.96× | 1.01× |
| 1,000,000 | 5.276 | 189.53M | 4.779 | 209.26M | 4.568 | 0.87× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.121 | 1.48× |
| 1 | 5 | 0.281 | 0.483 | 1.72× |
| 1 | 10 | 0.515 | 0.884 | 1.71× |
| 10 | 1 | 0.056 | 0.095 | 1.71× |
| 10 | 5 | 0.215 | 0.402 | 1.87× |
| 10 | 10 | 0.470 | 0.858 | 1.82× |
| 100 | 1 | 0.052 | 0.083 | 1.59× |
| 100 | 5 | 0.241 | 0.406 | 1.69× |
| 100 | 10 | 0.490 | 0.880 | 1.80× |
| 1,000 | 1 | 0.051 | 0.085 | 1.67× |
| 1,000 | 5 | 0.229 | 0.446 | 1.95× |
| 1,000 | 10 | 0.478 | 0.907 | 1.90× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
