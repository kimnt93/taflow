# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.25M | 0.004 | 248.60M | 0.031 | 5.44× | 7.67× |
| 10,000 | 0.031 | 321.95M | 0.029 | 339.56M | 0.044 | 1.41× | 1.49× |
| 100,000 | 0.301 | 332.64M | 0.258 | 387.34M | 0.158 | 0.53× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.140 | 1.61× |
| 1 | 5 | 0.243 | 0.458 | 1.89× |
| 1 | 10 | 0.386 | 0.908 | 2.35× |
| 10 | 1 | 0.046 | 0.095 | 2.07× |
| 10 | 5 | 0.193 | 0.464 | 2.41× |
| 10 | 10 | 0.398 | 0.906 | 2.27× |
| 100 | 1 | 0.042 | 0.083 | 1.96× |
| 100 | 5 | 0.197 | 0.428 | 2.17× |
| 100 | 10 | 0.417 | 0.933 | 2.24× |
| 1,000 | 1 | 0.047 | 0.086 | 1.82× |
| 1,000 | 5 | 0.215 | 0.414 | 1.93× |
| 1,000 | 10 | 0.434 | 0.876 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
