# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 175.06M | 0.004 | 257.78M | 0.034 | 6.00× | 8.83× |
| 10,000 | 0.013 | 791.48M | 0.010 | 1.05G | 0.039 | 3.07× | 4.07× |
| 100,000 | 0.093 | 1.08G | 0.054 | 1.87G | 0.086 | 0.92× | 1.60× |
| 1,000,000 | 2.116 | 472.68M | 1.341 | 745.69M | 1.317 | 0.62× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.139 | 1.75× |
| 1 | 5 | 0.298 | 0.447 | 1.50× |
| 1 | 10 | 0.636 | 1.029 | 1.62× |
| 10 | 1 | 0.054 | 0.093 | 1.71× |
| 10 | 5 | 0.246 | 0.431 | 1.75× |
| 10 | 10 | 0.491 | 1.021 | 2.08× |
| 100 | 1 | 0.058 | 0.084 | 1.45× |
| 100 | 5 | 0.250 | 0.452 | 1.81× |
| 100 | 10 | 0.489 | 1.000 | 2.04× |
| 1,000 | 1 | 0.063 | 0.110 | 1.75× |
| 1,000 | 5 | 0.274 | 0.503 | 1.84× |
| 1,000 | 10 | 0.483 | 1.083 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
