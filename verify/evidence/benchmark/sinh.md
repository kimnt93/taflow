# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.44M | 0.011 | 91.75M | 0.033 | 2.30× | 3.03× |
| 10,000 | 0.067 | 150.28M | 0.061 | 162.85M | 0.094 | 1.41× | 1.53× |
| 100,000 | 0.622 | 160.69M | 0.603 | 165.78M | 0.667 | 1.07× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.104 | 1.02× |
| 1 | 5 | 0.237 | 0.462 | 1.95× |
| 1 | 10 | 0.469 | 0.906 | 1.93× |
| 10 | 1 | 0.055 | 0.091 | 1.66× |
| 10 | 5 | 0.253 | 0.441 | 1.74× |
| 10 | 10 | 0.474 | 0.869 | 1.83× |
| 100 | 1 | 0.047 | 0.083 | 1.78× |
| 100 | 5 | 0.221 | 0.457 | 2.06× |
| 100 | 10 | 0.529 | 0.989 | 1.87× |
| 1,000 | 1 | 0.064 | 0.107 | 1.67× |
| 1,000 | 5 | 0.268 | 0.456 | 1.70× |
| 1,000 | 10 | 0.575 | 2.277 | 3.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
