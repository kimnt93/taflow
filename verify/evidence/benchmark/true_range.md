# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 334.98M | 0.002 | 631.85M | 0.028 | 9.44× | 17.81× |
| 10,000 | 0.012 | 854.95M | 0.008 | 1.21G | 0.034 | 2.93× | 4.16× |
| 100,000 | 0.114 | 880.09M | 0.086 | 1.16G | 0.099 | 0.87× | 1.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.110 | 1.72× |
| 1 | 5 | 0.255 | 0.438 | 1.72× |
| 1 | 10 | 0.374 | 0.965 | 2.58× |
| 10 | 1 | 0.043 | 0.089 | 2.05× |
| 10 | 5 | 0.173 | 0.415 | 2.40× |
| 10 | 10 | 0.376 | 0.877 | 2.33× |
| 100 | 1 | 0.039 | 0.086 | 2.21× |
| 100 | 5 | 0.205 | 0.448 | 2.18× |
| 100 | 10 | 0.384 | 0.867 | 2.26× |
| 1,000 | 1 | 0.041 | 0.090 | 2.18× |
| 1,000 | 5 | 0.195 | 0.420 | 2.16× |
| 1,000 | 10 | 0.438 | 0.944 | 2.16× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
