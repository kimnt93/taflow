# HigherHigh benchmark (`higher high relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.82M | 0.041 | 24.39M | 0.017 | 0.37× | 0.41× |
| 10,000 | 0.337 | 29.69M | 0.316 | 31.61M | 0.024 | 0.07× | 0.08× |
| 100,000 | 3.440 | 29.07M | 3.253 | 30.74M | 0.100 | 0.03× | 0.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.116 | 0.069 | 0.60× |
| 1 | 5 | 0.356 | 0.338 | 0.95× |
| 1 | 10 | 0.585 | 0.664 | 1.13× |
| 10 | 1 | 0.068 | 0.067 | 0.98× |
| 10 | 5 | 0.291 | 0.320 | 1.10× |
| 10 | 10 | 0.584 | 0.673 | 1.15× |
| 100 | 1 | 0.071 | 0.064 | 0.89× |
| 100 | 5 | 0.307 | 0.319 | 1.04× |
| 100 | 10 | 0.625 | 0.674 | 1.08× |
| 1,000 | 1 | 0.100 | 0.069 | 0.69× |
| 1,000 | 5 | 0.308 | 0.381 | 1.23× |
| 1,000 | 10 | 0.603 | 0.768 | 1.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
