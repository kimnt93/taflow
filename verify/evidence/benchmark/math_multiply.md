# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.004 | 227.60M | 0.003 | 301.75M | 0.032 | 7.33× | 9.72× |
| 10,000 | 0.010 | 1.03G | 0.007 | 1.47G | 0.032 | 3.26× | 4.67× |
| 100,000 | 0.066 | 1.51G | 0.040 | 2.52G | 0.069 | 1.04× | 1.74× |
| 1,000,000 | 1.074 | 930.78M | 1.062 | 941.43M | 0.958 | 0.89× | 0.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.153 | 0.120 | 0.79× |
| 1 | 5 | 0.255 | 0.460 | 1.80× |
| 1 | 10 | 0.494 | 0.896 | 1.82× |
| 10 | 1 | 0.048 | 0.088 | 1.84× |
| 10 | 5 | 0.220 | 0.423 | 1.92× |
| 10 | 10 | 0.451 | 0.872 | 1.93× |
| 100 | 1 | 0.047 | 0.086 | 1.81× |
| 100 | 5 | 0.220 | 0.432 | 1.97× |
| 100 | 10 | 0.513 | 0.962 | 1.87× |
| 1,000 | 1 | 0.054 | 0.087 | 1.59× |
| 1,000 | 5 | 0.238 | 0.449 | 1.89× |
| 1,000 | 10 | 0.519 | 0.974 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
