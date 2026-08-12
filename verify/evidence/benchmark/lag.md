# Lag benchmark (`causal lag` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 188.92M | 0.005 | 217.76M | 0.028 | 5.29× | 6.10× |
| 10,000 | 0.032 | 313.43M | 0.027 | 369.76M | 0.030 | 0.93× | 1.10× |
| 100,000 | 0.257 | 389.34M | 0.241 | 415.01M | 0.067 | 0.26× | 0.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.098 | 1.52× |
| 1 | 5 | 0.318 | 0.425 | 1.34× |
| 1 | 10 | 0.454 | 0.851 | 1.88× |
| 10 | 1 | 0.054 | 0.086 | 1.59× |
| 10 | 5 | 0.249 | 0.457 | 1.83× |
| 10 | 10 | 0.473 | 0.884 | 1.87× |
| 100 | 1 | 0.050 | 0.089 | 1.78× |
| 100 | 5 | 0.210 | 0.443 | 2.11× |
| 100 | 10 | 0.528 | 0.882 | 1.67× |
| 1,000 | 1 | 0.052 | 0.084 | 1.63× |
| 1,000 | 5 | 0.213 | 0.421 | 1.98× |
| 1,000 | 10 | 0.469 | 0.946 | 2.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
