# RelativeMomentumIndex benchmark (`RMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.47M | 0.054 | 18.43M | 0.173 | 3.02× | 3.19× |
| 10,000 | 0.467 | 21.43M | 0.431 | 23.20M | 0.535 | 1.15× | 1.24× |
| 100,000 | 4.405 | 22.70M | 4.462 | 22.41M | 3.876 | 0.88× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.263 | 2.58× |
| 1 | 5 | 0.400 | 1.126 | 2.81× |
| 1 | 10 | 0.601 | 2.507 | 4.17× |
| 10 | 1 | 0.082 | 0.221 | 2.68× |
| 10 | 5 | 0.289 | 1.045 | 3.62× |
| 10 | 10 | 0.608 | 2.451 | 4.03× |
| 100 | 1 | 0.076 | 0.217 | 2.87× |
| 100 | 5 | 0.294 | 1.068 | 3.64× |
| 100 | 10 | 0.606 | 2.476 | 4.08× |
| 1,000 | 1 | 0.125 | 0.258 | 2.07× |
| 1,000 | 5 | 0.321 | 1.288 | 4.01× |
| 1,000 | 10 | 0.657 | 2.823 | 4.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
