# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 434.47M | 0.001 | 723.08M | 0.028 | 12.13× | 20.18× |
| 10,000 | 0.010 | 953.32M | 0.008 | 1.30G | 0.040 | 3.85× | 5.24× |
| 100,000 | 0.096 | 1.05G | 0.069 | 1.45G | 0.167 | 1.75× | 2.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.101 | 1.65× |
| 1 | 5 | 0.300 | 0.424 | 1.41× |
| 1 | 10 | 0.384 | 0.946 | 2.46× |
| 10 | 1 | 0.041 | 0.085 | 2.06× |
| 10 | 5 | 0.175 | 0.439 | 2.51× |
| 10 | 10 | 0.364 | 0.863 | 2.37× |
| 100 | 1 | 0.039 | 0.087 | 2.24× |
| 100 | 5 | 0.193 | 0.438 | 2.27× |
| 100 | 10 | 0.394 | 0.860 | 2.18× |
| 1,000 | 1 | 0.042 | 0.087 | 2.07× |
| 1,000 | 5 | 0.191 | 0.393 | 2.06× |
| 1,000 | 10 | 0.410 | 0.918 | 2.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
