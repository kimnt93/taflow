# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.18M | 0.008 | 127.54M | 0.030 | 2.70× | 3.78× |
| 10,000 | 0.053 | 187.61M | 0.046 | 219.43M | 0.041 | 0.77× | 0.91× |
| 100,000 | 0.465 | 214.87M | 0.432 | 231.31M | 0.119 | 0.26× | 0.28× |
| 1,000,000 | 4.924 | 203.09M | 3.863 | 258.85M | 1.800 | 0.37× | 0.47× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.112 | 1.47× |
| 1 | 5 | 0.312 | 0.462 | 1.48× |
| 1 | 10 | 0.525 | 1.081 | 2.06× |
| 10 | 1 | 0.083 | 0.119 | 1.43× |
| 10 | 5 | 0.273 | 0.472 | 1.73× |
| 10 | 10 | 0.527 | 0.950 | 1.80× |
| 100 | 1 | 0.054 | 0.087 | 1.60× |
| 100 | 5 | 0.294 | 0.502 | 1.71× |
| 100 | 10 | 0.628 | 0.926 | 1.47× |
| 1,000 | 1 | 0.067 | 0.102 | 1.53× |
| 1,000 | 5 | 0.289 | 0.513 | 1.77× |
| 1,000 | 10 | 0.548 | 0.940 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
