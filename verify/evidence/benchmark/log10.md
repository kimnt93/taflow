# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.94M | 0.012 | 81.63M | 0.041 | 3.26× | 3.38× |
| 10,000 | 0.104 | 96.55M | 0.093 | 107.48M | 0.123 | 1.19× | 1.32× |
| 100,000 | 0.901 | 110.98M | 0.818 | 122.30M | 1.024 | 1.14× | 1.25× |
| 1,000,000 | 9.572 | 104.47M | 9.284 | 107.71M | 9.144 | 0.96× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.062 | 0.114 | 1.84× |
| 1 | 5 | 0.320 | 0.449 | 1.40× |
| 1 | 10 | 0.464 | 0.939 | 2.02× |
| 10 | 1 | 0.056 | 0.085 | 1.53× |
| 10 | 5 | 0.226 | 0.421 | 1.86× |
| 10 | 10 | 0.526 | 0.932 | 1.77× |
| 100 | 1 | 0.057 | 0.088 | 1.55× |
| 100 | 5 | 0.236 | 0.482 | 2.05× |
| 100 | 10 | 0.494 | 0.956 | 1.93× |
| 1,000 | 1 | 0.061 | 0.103 | 1.67× |
| 1,000 | 5 | 0.224 | 0.487 | 2.17× |
| 1,000 | 10 | 0.565 | 0.980 | 1.73× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
