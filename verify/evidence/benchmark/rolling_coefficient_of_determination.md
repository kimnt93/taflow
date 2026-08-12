# RollingCoefficientOfDetermination benchmark (`rolling squared correlation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.96M | 0.054 | 18.40M | 0.327 | 5.88× | 6.02× |
| 10,000 | 0.514 | 19.47M | 0.503 | 19.89M | 2.048 | 3.99× | 4.07× |
| 100,000 | 5.085 | 19.67M | 4.973 | 20.11M | 26.316 | 5.18× | 5.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.159 | 2.31× |
| 1 | 5 | 0.346 | 0.755 | 2.18× |
| 1 | 10 | 0.519 | 1.375 | 2.65× |
| 10 | 1 | 0.047 | 0.133 | 2.80× |
| 10 | 5 | 0.234 | 0.665 | 2.84× |
| 10 | 10 | 0.548 | 1.423 | 2.60× |
| 100 | 1 | 0.059 | 0.225 | 3.78× |
| 100 | 5 | 0.252 | 1.205 | 4.78× |
| 100 | 10 | 0.540 | 2.489 | 4.61× |
| 1,000 | 1 | 0.108 | 0.398 | 3.68× |
| 1,000 | 5 | 0.293 | 1.663 | 5.68× |
| 1,000 | 10 | 0.562 | 3.501 | 6.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
