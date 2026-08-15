# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 231.34M | 0.003 | 304.78M | 0.030 | 6.95× | 9.16× |
| 10,000 | 0.029 | 345.19M | 0.026 | 382.81M | 0.055 | 1.90× | 2.11× |
| 100,000 | 0.276 | 362.92M | 0.252 | 397.09M | 0.306 | 1.11× | 1.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.097 | 1.41× |
| 1 | 5 | 0.198 | 0.438 | 2.22× |
| 1 | 10 | 0.358 | 0.842 | 2.35× |
| 10 | 1 | 0.046 | 0.080 | 1.76× |
| 10 | 5 | 0.178 | 0.417 | 2.35× |
| 10 | 10 | 0.402 | 0.858 | 2.13× |
| 100 | 1 | 0.043 | 0.082 | 1.93× |
| 100 | 5 | 0.185 | 0.421 | 2.27× |
| 100 | 10 | 0.391 | 0.915 | 2.34× |
| 1,000 | 1 | 0.049 | 0.089 | 1.81× |
| 1,000 | 5 | 0.198 | 0.440 | 2.22× |
| 1,000 | 10 | 0.400 | 0.890 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
