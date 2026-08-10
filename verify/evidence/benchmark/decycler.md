# Decycler benchmark (`Decycler` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.97M | 0.010 | 97.42M | 0.151 | 14.61× | 14.67× |
| 10,000 | 0.075 | 132.50M | 0.074 | 134.47M | 0.526 | 6.97× | 7.07× |
| 100,000 | 0.696 | 143.76M | 0.662 | 151.06M | 3.741 | 5.38× | 5.65× |
| 1,000,000 | 7.214 | 138.63M | 6.508 | 153.66M | 39.643 | 5.50× | 6.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.164 | 0.852 | 5.20× |
| 1 | 5 | 0.311 | 1.027 | 3.30× |
| 1 | 10 | 0.449 | 2.104 | 4.68× |
| 10 | 1 | 0.054 | 0.192 | 3.53× |
| 10 | 5 | 0.224 | 0.938 | 4.19× |
| 10 | 10 | 0.474 | 2.223 | 4.69× |
| 100 | 1 | 0.054 | 0.201 | 3.73× |
| 100 | 5 | 0.235 | 0.958 | 4.07× |
| 100 | 10 | 0.530 | 2.371 | 4.47× |
| 1,000 | 1 | 0.058 | 0.232 | 4.01× |
| 1,000 | 5 | 0.265 | 1.196 | 4.51× |
| 1,000 | 10 | 0.557 | 2.611 | 4.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
