# MathAtan benchmark (`ATAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.68M | 0.009 | 115.45M | 0.033 | 3.16× | 3.77× |
| 10,000 | 0.064 | 156.50M | 0.060 | 165.31M | 0.088 | 1.37× | 1.45× |
| 100,000 | 0.643 | 155.47M | 0.606 | 165.10M | 0.635 | 0.99× | 1.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.172 | 0.101 | 0.59× |
| 1 | 5 | 0.295 | 0.418 | 1.41× |
| 1 | 10 | 0.453 | 0.870 | 1.92× |
| 10 | 1 | 0.054 | 0.087 | 1.62× |
| 10 | 5 | 0.252 | 0.477 | 1.89× |
| 10 | 10 | 0.516 | 0.866 | 1.68× |
| 100 | 1 | 0.052 | 0.086 | 1.66× |
| 100 | 5 | 0.248 | 0.431 | 1.74× |
| 100 | 10 | 0.492 | 0.880 | 1.79× |
| 1,000 | 1 | 0.058 | 0.096 | 1.65× |
| 1,000 | 5 | 0.231 | 0.455 | 1.97× |
| 1,000 | 10 | 0.497 | 1.034 | 2.08× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
