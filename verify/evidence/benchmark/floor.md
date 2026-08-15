# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 530.50M | 0.001 | 1.13G | 0.030 | 15.73× | 33.62× |
| 10,000 | 0.006 | 1.74G | 0.003 | 3.11G | 0.048 | 8.37× | 14.98× |
| 100,000 | 0.058 | 1.72G | 0.034 | 2.95G | 0.191 | 3.28× | 5.64× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.125 | 1.81× |
| 1 | 5 | 0.281 | 0.454 | 1.62× |
| 1 | 10 | 0.505 | 0.936 | 1.85× |
| 10 | 1 | 0.068 | 0.117 | 1.71× |
| 10 | 5 | 0.204 | 0.460 | 2.26× |
| 10 | 10 | 0.441 | 0.989 | 2.24× |
| 100 | 1 | 0.050 | 0.098 | 1.94× |
| 100 | 5 | 0.230 | 0.447 | 1.94× |
| 100 | 10 | 0.378 | 1.016 | 2.69× |
| 1,000 | 1 | 0.059 | 0.113 | 1.93× |
| 1,000 | 5 | 0.221 | 0.518 | 2.34× |
| 1,000 | 10 | 0.411 | 0.944 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
