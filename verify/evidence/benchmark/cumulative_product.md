# CumulativeProduct benchmark (`numpy.cumprod` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 321.38M | 0.002 | 411.34M | 0.018 | 5.87× | 7.51× |
| 10,000 | 0.013 | 743.62M | 0.011 | 944.08M | 0.037 | 2.78× | 3.52× |
| 100,000 | 0.119 | 839.87M | 0.101 | 986.10M | 0.224 | 1.88× | 2.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.075 | 0.91× |
| 1 | 5 | 0.235 | 0.319 | 1.35× |
| 1 | 10 | 0.374 | 0.683 | 1.83× |
| 10 | 1 | 0.054 | 0.070 | 1.29× |
| 10 | 5 | 0.186 | 0.296 | 1.59× |
| 10 | 10 | 0.375 | 0.657 | 1.75× |
| 100 | 1 | 0.047 | 0.071 | 1.53× |
| 100 | 5 | 0.178 | 0.304 | 1.71× |
| 100 | 10 | 0.404 | 0.634 | 1.57× |
| 1,000 | 1 | 0.044 | 0.077 | 1.76× |
| 1,000 | 5 | 0.210 | 0.364 | 1.74× |
| 1,000 | 10 | 0.388 | 0.806 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
