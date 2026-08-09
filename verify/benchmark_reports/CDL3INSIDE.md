# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 97.17M | 0.008 | 118.28M | 0.044 | 4.28× | 5.20× |
| 10,000 | 0.100 | 99.84M | 0.096 | 103.75M | 0.139 | 1.39× | 1.44× |
| 100,000 | 1.041 | 96.02M | 1.019 | 98.17M | 1.074 | 1.03× | 1.05× |
| 1,000,000 | 10.382 | 96.32M | 10.623 | 94.13M | 10.799 | 1.04× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.132 | 0.137 | 1.04× |
| 1 | 5 | 0.359 | 0.484 | 1.35× |
| 1 | 10 | 0.483 | 0.924 | 1.91× |
| 10 | 1 | 0.052 | 0.093 | 1.79× |
| 10 | 5 | 0.244 | 0.437 | 1.79× |
| 10 | 10 | 0.522 | 0.953 | 1.83× |
| 100 | 1 | 0.059 | 0.091 | 1.55× |
| 100 | 5 | 0.241 | 0.436 | 1.81× |
| 100 | 10 | 0.513 | 0.913 | 1.78× |
| 1,000 | 1 | 0.068 | 0.104 | 1.52× |
| 1,000 | 5 | 0.259 | 0.496 | 1.92× |
| 1,000 | 10 | 0.519 | 1.025 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
