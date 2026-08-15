# MathCos benchmark (`COS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.69M | 0.010 | 96.97M | 0.042 | 3.67× | 4.06× |
| 10,000 | 0.157 | 63.75M | 0.143 | 69.93M | 0.184 | 1.17× | 1.29× |
| 100,000 | 1.607 | 62.25M | 1.606 | 62.28M | 1.633 | 1.02× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.158 | 1.43× |
| 1 | 5 | 0.249 | 0.452 | 1.81× |
| 1 | 10 | 0.441 | 0.957 | 2.17× |
| 10 | 1 | 0.044 | 0.081 | 1.82× |
| 10 | 5 | 0.215 | 0.468 | 2.17× |
| 10 | 10 | 0.421 | 0.959 | 2.28× |
| 100 | 1 | 0.050 | 0.086 | 1.73× |
| 100 | 5 | 0.216 | 0.507 | 2.35× |
| 100 | 10 | 0.450 | 1.002 | 2.22× |
| 1,000 | 1 | 0.063 | 0.120 | 1.90× |
| 1,000 | 5 | 0.248 | 0.548 | 2.21× |
| 1,000 | 10 | 0.480 | 1.130 | 2.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
