# MathDivide benchmark (`DIV` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 208.14M | 0.003 | 298.23M | 0.029 | 6.04× | 8.66× |
| 10,000 | 0.012 | 839.11M | 0.008 | 1.26G | 0.033 | 2.76× | 4.14× |
| 100,000 | 0.074 | 1.35G | 0.047 | 2.15G | 0.080 | 1.08× | 1.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.097 | 0.85× |
| 1 | 5 | 0.394 | 0.457 | 1.16× |
| 1 | 10 | 0.473 | 0.893 | 1.89× |
| 10 | 1 | 0.050 | 0.092 | 1.82× |
| 10 | 5 | 0.214 | 0.442 | 2.07× |
| 10 | 10 | 0.453 | 0.890 | 1.96× |
| 100 | 1 | 0.052 | 0.084 | 1.62× |
| 100 | 5 | 0.231 | 0.417 | 1.80× |
| 100 | 10 | 0.468 | 0.877 | 1.87× |
| 1,000 | 1 | 0.054 | 0.087 | 1.61× |
| 1,000 | 5 | 0.224 | 0.417 | 1.86× |
| 1,000 | 10 | 0.473 | 0.901 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
