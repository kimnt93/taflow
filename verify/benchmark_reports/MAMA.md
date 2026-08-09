# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 17.07M | 0.060 | 16.79M | 0.093 | 1.59× | 1.57× |
| 10,000 | 0.549 | 18.22M | 0.555 | 18.02M | 0.561 | 1.02× | 1.01× |
| 100,000 | 6.161 | 16.23M | 5.453 | 18.34M | 5.354 | 0.87× | 0.98× |
| 1,000,000 | 54.772 | 18.26M | 56.972 | 17.55M | 52.581 | 0.96× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.150 | 1.20× |
| 1 | 5 | 0.267 | 0.571 | 2.14× |
| 1 | 10 | 0.569 | 1.292 | 2.27× |
| 10 | 1 | 0.053 | 0.112 | 2.11× |
| 10 | 5 | 0.280 | 0.583 | 2.08× |
| 10 | 10 | 0.587 | 1.298 | 2.21× |
| 100 | 1 | 0.064 | 0.112 | 1.75× |
| 100 | 5 | 0.273 | 0.542 | 1.98× |
| 100 | 10 | 0.585 | 1.211 | 2.07× |
| 1,000 | 1 | 0.109 | 0.162 | 1.49× |
| 1,000 | 5 | 0.320 | 0.842 | 2.63× |
| 1,000 | 10 | 0.627 | 1.702 | 2.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
