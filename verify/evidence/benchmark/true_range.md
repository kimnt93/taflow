# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.40M | 0.046 | 21.84M | 0.027 | 0.50× | 0.60× |
| 10,000 | 0.374 | 26.77M | 0.370 | 27.03M | 0.033 | 0.09× | 0.09× |
| 100,000 | 3.536 | 28.28M | 3.733 | 26.79M | 0.085 | 0.02× | 0.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.108 | 1.21× |
| 1 | 5 | 0.369 | 0.461 | 1.25× |
| 1 | 10 | 0.627 | 0.889 | 1.42× |
| 10 | 1 | 0.069 | 0.086 | 1.26× |
| 10 | 5 | 0.285 | 0.411 | 1.44× |
| 10 | 10 | 0.606 | 0.899 | 1.48× |
| 100 | 1 | 0.078 | 0.090 | 1.15× |
| 100 | 5 | 0.295 | 0.414 | 1.40× |
| 100 | 10 | 0.635 | 0.861 | 1.36× |
| 1,000 | 1 | 0.104 | 0.091 | 0.87× |
| 1,000 | 5 | 0.301 | 0.409 | 1.36× |
| 1,000 | 10 | 0.599 | 0.880 | 1.47× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
