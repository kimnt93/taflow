# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 213.31M | 0.003 | 304.07M | 0.029 | 6.25× | 8.91× |
| 10,000 | 0.010 | 966.55M | 0.007 | 1.39G | 0.034 | 3.24× | 4.66× |
| 100,000 | 0.066 | 1.52G | 0.041 | 2.44G | 0.068 | 1.03× | 1.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.089 | 0.160 | 1.79× |
| 1 | 5 | 0.404 | 0.466 | 1.16× |
| 1 | 10 | 0.487 | 0.942 | 1.93× |
| 10 | 1 | 0.049 | 0.089 | 1.81× |
| 10 | 5 | 0.229 | 0.446 | 1.95× |
| 10 | 10 | 0.452 | 0.906 | 2.01× |
| 100 | 1 | 0.049 | 0.100 | 2.03× |
| 100 | 5 | 0.224 | 0.426 | 1.90× |
| 100 | 10 | 0.496 | 0.914 | 1.85× |
| 1,000 | 1 | 0.054 | 0.087 | 1.63× |
| 1,000 | 5 | 0.227 | 0.426 | 1.88× |
| 1,000 | 10 | 0.474 | 0.894 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
