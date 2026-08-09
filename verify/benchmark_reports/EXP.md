# MathExp benchmark (`EXP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 112.42M | 0.008 | 130.86M | 0.031 | 3.51× | 4.08× |
| 10,000 | 0.057 | 176.34M | 0.054 | 186.85M | 0.071 | 1.26× | 1.33× |
| 100,000 | 0.530 | 188.79M | 0.512 | 195.16M | 0.475 | 0.90× | 0.93× |
| 1,000,000 | 6.611 | 151.27M | 5.703 | 175.36M | 4.519 | 0.68× | 0.79× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.137 | 0.133 | 0.97× |
| 1 | 5 | 0.397 | 0.495 | 1.25× |
| 1 | 10 | 0.459 | 0.880 | 1.92× |
| 10 | 1 | 0.052 | 0.088 | 1.67× |
| 10 | 5 | 0.220 | 0.417 | 1.90× |
| 10 | 10 | 0.475 | 0.902 | 1.90× |
| 100 | 1 | 0.051 | 0.088 | 1.73× |
| 100 | 5 | 0.216 | 0.423 | 1.96× |
| 100 | 10 | 0.489 | 0.877 | 1.79× |
| 1,000 | 1 | 0.055 | 0.095 | 1.71× |
| 1,000 | 5 | 0.241 | 0.447 | 1.85× |
| 1,000 | 10 | 0.515 | 0.963 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
