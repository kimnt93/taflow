# MathCot benchmark (`numpy.tan reciprocal` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.69M | 0.013 | 77.89M | 0.023 | 1.69× | 1.81× |
| 10,000 | 0.121 | 82.31M | 0.119 | 84.10M | 0.133 | 1.10× | 1.12× |
| 100,000 | 1.240 | 80.62M | 1.195 | 83.69M | 1.212 | 0.98× | 1.01× |
| 1,000,000 | 12.250 | 81.63M | 12.522 | 79.86M | 15.596 | 1.27× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.062 | 0.60× |
| 1 | 5 | 0.334 | 0.292 | 0.87× |
| 1 | 10 | 0.485 | 0.607 | 1.25× |
| 10 | 1 | 0.045 | 0.056 | 1.24× |
| 10 | 5 | 0.216 | 0.279 | 1.29× |
| 10 | 10 | 0.470 | 0.604 | 1.29× |
| 100 | 1 | 0.048 | 0.062 | 1.29× |
| 100 | 5 | 0.221 | 0.288 | 1.31× |
| 100 | 10 | 0.480 | 0.617 | 1.29× |
| 1,000 | 1 | 0.058 | 0.070 | 1.21× |
| 1,000 | 5 | 0.240 | 0.338 | 1.41× |
| 1,000 | 10 | 0.517 | 0.814 | 1.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
