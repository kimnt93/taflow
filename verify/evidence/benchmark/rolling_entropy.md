# RollingEntropy benchmark (`rolling Shannon entropy` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.755 | 1.32M | 0.751 | 1.33M | 0.049 | 0.07× | 0.07× |
| 10,000 | 8.025 | 1.25M | 9.080 | 1.10M | 0.155 | 0.02× | 0.02× |
| 100,000 | 78.810 | 1.27M | 78.555 | 1.27M | 1.038 | 0.01× | 0.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.109 | 1.07× |
| 1 | 5 | 0.262 | 0.416 | 1.59× |
| 1 | 10 | 0.497 | 0.839 | 1.69× |
| 10 | 1 | 0.051 | 0.087 | 1.73× |
| 10 | 5 | 0.228 | 0.417 | 1.83× |
| 10 | 10 | 0.487 | 0.828 | 1.70× |
| 100 | 1 | 0.117 | 0.123 | 1.05× |
| 100 | 5 | 0.293 | 0.567 | 1.94× |
| 100 | 10 | 0.558 | 1.152 | 2.06× |
| 1,000 | 1 | 0.857 | 0.126 | 0.15× |
| 1,000 | 5 | 0.978 | 0.638 | 0.65× |
| 1,000 | 10 | 1.569 | 1.458 | 0.93× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
