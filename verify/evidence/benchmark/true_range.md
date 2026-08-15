# TrueRange benchmark (`TRANGE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 168.33M | 0.004 | 233.24M | 0.029 | 4.85× | 6.72× |
| 10,000 | 0.034 | 290.76M | 0.032 | 312.48M | 0.035 | 1.01× | 1.09× |
| 100,000 | 0.345 | 289.77M | 0.293 | 341.71M | 0.104 | 0.30× | 0.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.067 | 0.105 | 1.58× |
| 1 | 5 | 0.331 | 0.448 | 1.35× |
| 1 | 10 | 0.369 | 0.941 | 2.55× |
| 10 | 1 | 0.051 | 0.112 | 2.19× |
| 10 | 5 | 0.226 | 0.476 | 2.11× |
| 10 | 10 | 0.391 | 0.896 | 2.29× |
| 100 | 1 | 0.043 | 0.092 | 2.11× |
| 100 | 5 | 0.198 | 0.424 | 2.15× |
| 100 | 10 | 0.431 | 0.910 | 2.11× |
| 1,000 | 1 | 0.045 | 0.088 | 1.96× |
| 1,000 | 5 | 0.189 | 0.407 | 2.15× |
| 1,000 | 10 | 0.405 | 0.948 | 2.34× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
