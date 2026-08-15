# CenterOfGravity benchmark (`CenterOfGravity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.95M | 0.018 | 54.13M | 0.181 | 9.02× | 9.78× |
| 10,000 | 0.179 | 56.01M | 0.175 | 57.24M | 0.625 | 3.50× | 3.58× |
| 100,000 | 1.818 | 55.00M | 1.665 | 60.06M | 5.044 | 2.77× | 3.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.222 | 3.83× |
| 1 | 5 | 0.247 | 0.973 | 3.94× |
| 1 | 10 | 0.383 | 2.128 | 5.56× |
| 10 | 1 | 0.046 | 0.190 | 4.14× |
| 10 | 5 | 0.199 | 0.934 | 4.69× |
| 10 | 10 | 0.454 | 2.204 | 4.85× |
| 100 | 1 | 0.049 | 0.193 | 3.95× |
| 100 | 5 | 0.193 | 0.978 | 5.08× |
| 100 | 10 | 0.473 | 2.174 | 4.59× |
| 1,000 | 1 | 0.065 | 0.236 | 3.61× |
| 1,000 | 5 | 0.189 | 1.244 | 6.60× |
| 1,000 | 10 | 0.421 | 2.571 | 6.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
