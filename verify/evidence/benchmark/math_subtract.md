# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 209.44M | 0.003 | 302.57M | 0.029 | 6.17× | 8.91× |
| 10,000 | 0.010 | 1.02G | 0.007 | 1.45G | 0.033 | 3.38× | 4.81× |
| 100,000 | 0.062 | 1.61G | 0.042 | 2.36G | 0.070 | 1.13× | 1.66× |
| 1,000,000 | 1.139 | 878.04M | 0.845 | 1.18G | 0.844 | 0.74× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.103 | 1.21× |
| 1 | 5 | 0.325 | 0.439 | 1.35× |
| 1 | 10 | 0.492 | 0.901 | 1.83× |
| 10 | 1 | 0.055 | 0.087 | 1.59× |
| 10 | 5 | 0.223 | 0.421 | 1.89× |
| 10 | 10 | 0.487 | 0.891 | 1.83× |
| 100 | 1 | 0.047 | 0.085 | 1.79× |
| 100 | 5 | 0.224 | 0.417 | 1.86× |
| 100 | 10 | 0.482 | 0.912 | 1.89× |
| 1,000 | 1 | 0.057 | 0.088 | 1.56× |
| 1,000 | 5 | 0.233 | 0.440 | 1.89× |
| 1,000 | 10 | 0.470 | 0.901 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
