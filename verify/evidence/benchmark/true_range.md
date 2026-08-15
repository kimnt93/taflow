# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 329.39M | 0.002 | 639.48M | 0.027 | 8.77× | 17.02× |
| 10,000 | 0.012 | 812.77M | 0.009 | 1.13G | 0.034 | 2.77× | 3.85× |
| 100,000 | 0.107 | 937.85M | 0.084 | 1.19G | 0.087 | 0.82× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.106 | 0.82× |
| 1 | 5 | 0.210 | 0.487 | 2.32× |
| 1 | 10 | 0.443 | 0.923 | 2.08× |
| 10 | 1 | 0.050 | 0.103 | 2.06× |
| 10 | 5 | 0.201 | 0.497 | 2.47× |
| 10 | 10 | 0.418 | 0.922 | 2.20× |
| 100 | 1 | 0.041 | 0.092 | 2.26× |
| 100 | 5 | 0.183 | 0.448 | 2.45× |
| 100 | 10 | 0.441 | 0.971 | 2.20× |
| 1,000 | 1 | 0.054 | 0.094 | 1.74× |
| 1,000 | 5 | 0.189 | 0.416 | 2.20× |
| 1,000 | 10 | 0.424 | 1.055 | 2.49× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
