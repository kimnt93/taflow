# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 461.13M | 0.001 | 735.52M | 0.029 | 13.40× | 21.37× |
| 10,000 | 0.008 | 1.23G | 0.006 | 1.78G | 0.039 | 4.83× | 6.97× |
| 100,000 | 0.069 | 1.45G | 0.049 | 2.03G | 0.127 | 1.84× | 2.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.103 | 1.01× |
| 1 | 5 | 0.263 | 0.432 | 1.65× |
| 1 | 10 | 0.386 | 0.892 | 2.31× |
| 10 | 1 | 0.046 | 0.085 | 1.85× |
| 10 | 5 | 0.190 | 0.428 | 2.25× |
| 10 | 10 | 0.398 | 0.902 | 2.26× |
| 100 | 1 | 0.041 | 0.085 | 2.06× |
| 100 | 5 | 0.176 | 0.434 | 2.46× |
| 100 | 10 | 0.395 | 0.887 | 2.24× |
| 1,000 | 1 | 0.044 | 0.097 | 2.22× |
| 1,000 | 5 | 0.181 | 0.457 | 2.53× |
| 1,000 | 10 | 0.393 | 0.957 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
