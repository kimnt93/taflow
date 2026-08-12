# MathAdd benchmark (`ADD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 220.27M | 0.003 | 305.81M | 0.029 | 6.43× | 8.93× |
| 10,000 | 0.011 | 946.59M | 0.007 | 1.40G | 0.032 | 3.02× | 4.46× |
| 100,000 | 0.065 | 1.53G | 0.042 | 2.36G | 0.068 | 1.04× | 1.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.073 | 0.109 | 1.49× |
| 1 | 5 | 0.331 | 0.451 | 1.36× |
| 1 | 10 | 0.461 | 0.895 | 1.94× |
| 10 | 1 | 0.051 | 0.087 | 1.72× |
| 10 | 5 | 0.225 | 0.436 | 1.94× |
| 10 | 10 | 0.469 | 0.880 | 1.88× |
| 100 | 1 | 0.049 | 0.084 | 1.71× |
| 100 | 5 | 0.224 | 0.415 | 1.85× |
| 100 | 10 | 0.485 | 0.895 | 1.84× |
| 1,000 | 1 | 0.048 | 0.084 | 1.74× |
| 1,000 | 5 | 0.228 | 0.428 | 1.87× |
| 1,000 | 10 | 0.498 | 0.894 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
