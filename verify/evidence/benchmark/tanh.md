# MathTanh benchmark (`TANH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.86M | 0.005 | 198.40M | 0.029 | 4.93× | 5.76× |
| 10,000 | 0.031 | 319.98M | 0.031 | 327.64M | 0.055 | 1.76× | 1.80× |
| 100,000 | 0.300 | 333.71M | 0.247 | 404.58M | 0.287 | 0.96× | 1.16× |
| 1,000,000 | 3.000 | 333.30M | 2.537 | 394.11M | 2.604 | 0.87× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.131 | 1.28× |
| 1 | 5 | 0.332 | 0.438 | 1.32× |
| 1 | 10 | 0.441 | 0.825 | 1.87× |
| 10 | 1 | 0.046 | 0.085 | 1.86× |
| 10 | 5 | 0.222 | 0.394 | 1.77× |
| 10 | 10 | 0.455 | 0.878 | 1.93× |
| 100 | 1 | 0.052 | 0.093 | 1.79× |
| 100 | 5 | 0.217 | 0.408 | 1.88× |
| 100 | 10 | 0.457 | 0.882 | 1.93× |
| 1,000 | 1 | 0.050 | 0.104 | 2.09× |
| 1,000 | 5 | 0.238 | 0.436 | 1.83× |
| 1,000 | 10 | 0.524 | 0.966 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
