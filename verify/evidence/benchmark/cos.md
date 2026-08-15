# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 88.43M | 0.010 | 98.10M | 0.040 | 3.50× | 3.88× |
| 10,000 | 0.156 | 63.99M | 0.147 | 67.98M | 0.172 | 1.10× | 1.17× |
| 100,000 | 1.486 | 67.27M | 1.452 | 68.86M | 1.481 | 1.00× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.130 | 1.67× |
| 1 | 5 | 0.252 | 0.452 | 1.79× |
| 1 | 10 | 0.386 | 0.911 | 2.36× |
| 10 | 1 | 0.042 | 0.086 | 2.03× |
| 10 | 5 | 0.185 | 0.424 | 2.29× |
| 10 | 10 | 0.413 | 0.878 | 2.13× |
| 100 | 1 | 0.044 | 0.091 | 2.08× |
| 100 | 5 | 0.191 | 0.405 | 2.12× |
| 100 | 10 | 0.392 | 0.902 | 2.30× |
| 1,000 | 1 | 0.057 | 0.102 | 1.78× |
| 1,000 | 5 | 0.194 | 0.491 | 2.53× |
| 1,000 | 10 | 0.410 | 1.026 | 2.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
