# WilliamsAccumulationDistribution benchmark (`Wad` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.054 | 18.44M | 0.046 | 21.82M | 0.184 | 3.40× | 4.02× |
| 10,000 | 0.383 | 26.13M | 0.370 | 27.02M | 1.040 | 2.72× | 2.81× |
| 100,000 | 3.625 | 27.59M | 3.528 | 28.35M | 9.439 | 2.60× | 2.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.124 | 0.171 | 1.38× |
| 1 | 5 | 0.428 | 1.148 | 2.68× |
| 1 | 10 | 0.623 | 1.846 | 2.97× |
| 10 | 1 | 0.072 | 0.163 | 2.27× |
| 10 | 5 | 0.307 | 0.801 | 2.61× |
| 10 | 10 | 0.619 | 1.909 | 3.09× |
| 100 | 1 | 0.075 | 0.174 | 2.34× |
| 100 | 5 | 0.309 | 0.858 | 2.78× |
| 100 | 10 | 0.655 | 2.133 | 3.25× |
| 1,000 | 1 | 0.125 | 0.264 | 2.11× |
| 1,000 | 5 | 0.316 | 1.308 | 4.14× |
| 1,000 | 10 | 0.661 | 2.656 | 4.02× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
