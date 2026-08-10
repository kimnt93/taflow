# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 215.87M | 0.003 | 313.60M | 0.027 | 5.85× | 8.50× |
| 10,000 | 0.010 | 1.01G | 0.007 | 1.50G | 0.031 | 3.17× | 4.72× |
| 100,000 | 0.062 | 1.61G | 0.040 | 2.48G | 0.076 | 1.22× | 1.87× |
| 1,000,000 | 1.313 | 761.35M | 0.800 | 1.25G | 0.852 | 0.65× | 1.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.101 | 0.97× |
| 1 | 5 | 0.379 | 0.509 | 1.34× |
| 1 | 10 | 0.497 | 0.894 | 1.80× |
| 10 | 1 | 0.048 | 0.084 | 1.74× |
| 10 | 5 | 0.217 | 0.413 | 1.90× |
| 10 | 10 | 0.486 | 0.912 | 1.88× |
| 100 | 1 | 0.049 | 0.084 | 1.73× |
| 100 | 5 | 0.216 | 0.413 | 1.91× |
| 100 | 10 | 0.469 | 0.885 | 1.89× |
| 1,000 | 1 | 0.048 | 0.085 | 1.80× |
| 1,000 | 5 | 0.230 | 0.423 | 1.84× |
| 1,000 | 10 | 0.489 | 0.932 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
