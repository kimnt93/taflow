# RollingMaximumDrawdown benchmark (`MaxDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.052 | 19.09M | 0.047 | 21.12M | 0.282 | 5.38× | 5.95× |
| 10,000 | 0.455 | 22.00M | 0.453 | 22.07M | 1.421 | 3.13× | 3.14× |
| 100,000 | 4.812 | 20.78M | 4.823 | 20.73M | 12.205 | 2.54× | 2.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.244 | 2.98× |
| 1 | 5 | 0.338 | 1.309 | 3.88× |
| 1 | 10 | 0.499 | 2.168 | 4.34× |
| 10 | 1 | 0.061 | 0.209 | 3.44× |
| 10 | 5 | 0.234 | 0.964 | 4.11× |
| 10 | 10 | 0.468 | 2.159 | 4.61× |
| 100 | 1 | 0.057 | 0.201 | 3.53× |
| 100 | 5 | 0.259 | 1.054 | 4.06× |
| 100 | 10 | 0.521 | 2.405 | 4.61× |
| 1,000 | 1 | 0.101 | 0.317 | 3.15× |
| 1,000 | 5 | 0.240 | 1.594 | 6.64× |
| 1,000 | 10 | 0.590 | 3.674 | 6.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
