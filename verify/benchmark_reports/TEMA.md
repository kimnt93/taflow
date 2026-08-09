# TripleExponentialMovingAverage benchmark (`TEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 76.08M | 0.012 | 82.66M | 0.044 | 3.34× | 3.63× |
| 10,000 | 0.099 | 100.56M | 0.096 | 104.16M | 0.120 | 1.21× | 1.25× |
| 100,000 | 0.974 | 102.71M | 0.985 | 101.53M | 0.945 | 0.97× | 0.96× |
| 1,000,000 | 10.647 | 93.92M | 9.591 | 104.26M | 10.717 | 1.01× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.124 | 1.18× |
| 1 | 5 | 0.319 | 0.540 | 1.69× |
| 1 | 10 | 0.528 | 1.077 | 2.04× |
| 10 | 1 | 0.052 | 0.098 | 1.88× |
| 10 | 5 | 0.237 | 0.467 | 1.97× |
| 10 | 10 | 0.513 | 1.009 | 1.97× |
| 100 | 1 | 0.055 | 0.115 | 2.07× |
| 100 | 5 | 0.248 | 0.506 | 2.04× |
| 100 | 10 | 0.511 | 1.013 | 1.98× |
| 1,000 | 1 | 0.067 | 0.121 | 1.80× |
| 1,000 | 5 | 0.265 | 0.543 | 2.05× |
| 1,000 | 10 | 0.514 | 1.081 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
