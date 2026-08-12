# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 245.51M | 0.003 | 319.69M | 0.029 | 7.14× | 9.29× |
| 10,000 | 0.013 | 778.79M | 0.010 | 990.71M | 0.044 | 3.39× | 4.32× |
| 100,000 | 0.098 | 1.02G | 0.075 | 1.33G | 0.180 | 1.82× | 2.39× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.104 | 1.01× |
| 1 | 5 | 0.247 | 0.473 | 1.92× |
| 1 | 10 | 0.504 | 0.882 | 1.75× |
| 10 | 1 | 0.045 | 0.082 | 1.80× |
| 10 | 5 | 0.220 | 0.427 | 1.94× |
| 10 | 10 | 0.506 | 0.918 | 1.81× |
| 100 | 1 | 0.050 | 0.093 | 1.87× |
| 100 | 5 | 0.219 | 0.415 | 1.90× |
| 100 | 10 | 0.545 | 1.043 | 1.91× |
| 1,000 | 1 | 0.058 | 0.101 | 1.74× |
| 1,000 | 5 | 0.260 | 0.478 | 1.84× |
| 1,000 | 10 | 0.493 | 0.915 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
