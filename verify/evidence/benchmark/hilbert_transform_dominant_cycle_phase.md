# HilbertTransformDominantCyclePhase benchmark (`HT_DCPHASE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.638 | 1.57M | 0.621 | 1.61M | 0.505 | 0.79× | 0.81× |
| 10,000 | 6.359 | 1.57M | 6.437 | 1.55M | 4.151 | 0.65× | 0.64× |
| 100,000 | 64.837 | 1.54M | 64.913 | 1.54M | 43.463 | 0.67× | 0.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.119 | 1.27× |
| 1 | 5 | 0.371 | 0.439 | 1.18× |
| 1 | 10 | 0.586 | 0.906 | 1.55× |
| 10 | 1 | 0.072 | 0.087 | 1.21× |
| 10 | 5 | 0.476 | 0.727 | 1.53× |
| 10 | 10 | 0.725 | 0.965 | 1.33× |
| 100 | 1 | 0.109 | 0.119 | 1.10× |
| 100 | 5 | 0.316 | 0.553 | 1.75× |
| 100 | 10 | 0.672 | 1.170 | 1.74× |
| 1,000 | 1 | 0.742 | 0.524 | 0.71× |
| 1,000 | 5 | 1.167 | 2.613 | 2.24× |
| 1,000 | 10 | 1.594 | 5.355 | 3.36× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
