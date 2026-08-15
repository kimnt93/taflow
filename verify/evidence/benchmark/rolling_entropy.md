# RollingEntropy benchmark (`rolling Shannon entropy` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.88M | 0.057 | 17.64M | 0.045 | 0.81× | 0.80× |
| 10,000 | 0.543 | 18.41M | 0.563 | 17.76M | 0.115 | 0.21× | 0.20× |
| 100,000 | 5.364 | 18.64M | 5.371 | 18.62M | 0.965 | 0.18× | 0.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.196 | 0.107 | 0.55× |
| 1 | 5 | 0.304 | 0.458 | 1.51× |
| 1 | 10 | 0.382 | 0.800 | 2.10× |
| 10 | 1 | 0.041 | 0.077 | 1.86× |
| 10 | 5 | 0.184 | 0.377 | 2.05× |
| 10 | 10 | 0.380 | 0.826 | 2.17× |
| 100 | 1 | 0.045 | 0.118 | 2.59× |
| 100 | 5 | 0.198 | 0.554 | 2.80× |
| 100 | 10 | 0.449 | 1.109 | 2.47× |
| 1,000 | 1 | 0.102 | 0.119 | 1.17× |
| 1,000 | 5 | 0.211 | 0.670 | 3.18× |
| 1,000 | 10 | 0.460 | 1.436 | 3.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
