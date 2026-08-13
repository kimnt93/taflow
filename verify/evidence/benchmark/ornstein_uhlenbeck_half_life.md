# OrnsteinUhlenbeckHalfLife benchmark (`rolling OU half life` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.305 | 3.28M | 0.282 | 3.54M | 0.286 | 0.94× | 1.01× |
| 10,000 | 2.857 | 3.50M | 2.839 | 3.52M | 1.578 | 0.55× | 0.56× |
| 100,000 | 29.177 | 3.43M | 28.191 | 3.55M | 17.275 | 0.59× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.134 | 1.24× |
| 1 | 5 | 0.425 | 0.431 | 1.01× |
| 1 | 10 | 0.599 | 0.823 | 1.37× |
| 10 | 1 | 0.069 | 0.082 | 1.19× |
| 10 | 5 | 0.288 | 0.400 | 1.39× |
| 10 | 10 | 0.597 | 0.803 | 1.35× |
| 100 | 1 | 0.087 | 0.252 | 2.91× |
| 100 | 5 | 0.297 | 1.219 | 4.11× |
| 100 | 10 | 0.619 | 2.506 | 4.05× |
| 1,000 | 1 | 0.376 | 0.370 | 0.98× |
| 1,000 | 5 | 0.588 | 1.594 | 2.71× |
| 1,000 | 10 | 0.984 | 3.299 | 3.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
