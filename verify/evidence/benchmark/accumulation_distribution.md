# AccumulationDistribution benchmark (`AD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 108.92M | 0.008 | 130.09M | 0.027 | 2.96× | 3.53× |
| 10,000 | 0.038 | 264.22M | 0.034 | 291.33M | 0.042 | 1.11× | 1.23× |
| 100,000 | 0.308 | 324.64M | 0.279 | 357.86M | 0.150 | 0.49× | 0.54× |
| 1,000,000 | 3.853 | 259.54M | 3.379 | 295.91M | 2.290 | 0.59× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.121 | 1.89× |
| 1 | 5 | 0.247 | 0.436 | 1.76× |
| 1 | 10 | 0.481 | 1.010 | 2.10× |
| 10 | 1 | 0.051 | 0.084 | 1.64× |
| 10 | 5 | 0.228 | 0.413 | 1.81× |
| 10 | 10 | 0.502 | 0.891 | 1.78× |
| 100 | 1 | 0.050 | 0.091 | 1.82× |
| 100 | 5 | 0.297 | 0.452 | 1.52× |
| 100 | 10 | 0.512 | 0.895 | 1.75× |
| 1,000 | 1 | 0.053 | 0.087 | 1.65× |
| 1,000 | 5 | 0.244 | 0.436 | 1.79× |
| 1,000 | 10 | 0.530 | 0.884 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
