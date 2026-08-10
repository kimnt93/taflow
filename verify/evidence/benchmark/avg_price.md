# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 111.41M | 0.007 | 136.87M | 0.028 | 3.11× | 3.82× |
| 10,000 | 0.029 | 341.56M | 0.026 | 383.41M | 0.033 | 1.14× | 1.28× |
| 100,000 | 0.233 | 430.10M | 0.214 | 466.47M | 0.085 | 0.37× | 0.40× |
| 1,000,000 | 3.109 | 321.67M | 2.572 | 388.76M | 1.371 | 0.44× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.158 | 1.63× |
| 1 | 5 | 0.441 | 0.475 | 1.08× |
| 1 | 10 | 0.491 | 0.893 | 1.82× |
| 10 | 1 | 0.054 | 0.087 | 1.60× |
| 10 | 5 | 0.230 | 0.436 | 1.89× |
| 10 | 10 | 0.504 | 0.920 | 1.83× |
| 100 | 1 | 0.049 | 0.086 | 1.73× |
| 100 | 5 | 0.240 | 0.423 | 1.76× |
| 100 | 10 | 0.504 | 0.922 | 1.83× |
| 1,000 | 1 | 0.057 | 0.086 | 1.52× |
| 1,000 | 5 | 0.264 | 0.437 | 1.65× |
| 1,000 | 10 | 0.522 | 0.939 | 1.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
