# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.08M | 0.009 | 110.21M | 0.036 | 3.60× | 3.96× |
| 10,000 | 0.070 | 142.97M | 0.066 | 151.35M | 0.098 | 1.40× | 1.48× |
| 100,000 | 0.706 | 141.70M | 0.681 | 146.80M | 0.717 | 1.02× | 1.05× |
| 1,000,000 | 7.533 | 132.74M | 6.830 | 146.40M | 6.566 | 0.87× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.116 | 1.04× |
| 1 | 5 | 0.270 | 0.424 | 1.57× |
| 1 | 10 | 0.445 | 0.857 | 1.93× |
| 10 | 1 | 0.047 | 0.082 | 1.74× |
| 10 | 5 | 0.255 | 0.452 | 1.77× |
| 10 | 10 | 0.466 | 0.855 | 1.83× |
| 100 | 1 | 0.046 | 0.086 | 1.87× |
| 100 | 5 | 0.239 | 0.415 | 1.74× |
| 100 | 10 | 0.503 | 0.919 | 1.83× |
| 1,000 | 1 | 0.066 | 0.094 | 1.43× |
| 1,000 | 5 | 0.243 | 0.441 | 1.81× |
| 1,000 | 10 | 0.485 | 0.963 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
