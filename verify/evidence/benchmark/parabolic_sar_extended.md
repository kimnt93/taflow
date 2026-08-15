# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.49M | 0.010 | 97.65M | 0.052 | 4.41× | 5.04× |
| 10,000 | 0.106 | 94.50M | 0.100 | 99.58M | 0.099 | 0.94× | 0.99× |
| 100,000 | 1.026 | 97.51M | 0.998 | 100.17M | 0.650 | 0.63× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.146 | 1.78× |
| 1 | 5 | 0.213 | 0.593 | 2.79× |
| 1 | 10 | 0.384 | 1.103 | 2.87× |
| 10 | 1 | 0.050 | 0.119 | 2.39× |
| 10 | 5 | 0.193 | 0.535 | 2.77× |
| 10 | 10 | 0.368 | 1.117 | 3.04× |
| 100 | 1 | 0.041 | 0.106 | 2.56× |
| 100 | 5 | 0.194 | 0.596 | 3.07× |
| 100 | 10 | 0.448 | 1.168 | 2.61× |
| 1,000 | 1 | 0.053 | 0.115 | 2.18× |
| 1,000 | 5 | 0.194 | 0.560 | 2.89× |
| 1,000 | 10 | 0.453 | 1.234 | 2.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
