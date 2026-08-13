# CenterOfGravity benchmark (`CenterOfGravity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.400 | 2.50M | 0.449 | 2.23M | 0.159 | 0.40× | 0.35× |
| 10,000 | 3.879 | 2.58M | 4.233 | 2.36M | 0.603 | 0.16× | 0.14× |
| 100,000 | 39.657 | 2.52M | 39.166 | 2.55M | 4.725 | 0.12× | 0.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.173 | 0.242 | 1.40× |
| 1 | 5 | 0.428 | 1.011 | 2.36× |
| 1 | 10 | 0.622 | 2.090 | 3.36× |
| 10 | 1 | 0.072 | 0.196 | 2.73× |
| 10 | 5 | 0.298 | 0.949 | 3.19× |
| 10 | 10 | 0.619 | 2.123 | 3.43× |
| 100 | 1 | 0.119 | 0.205 | 1.72× |
| 100 | 5 | 0.298 | 0.954 | 3.20× |
| 100 | 10 | 0.634 | 2.413 | 3.80× |
| 1,000 | 1 | 0.511 | 0.251 | 0.49× |
| 1,000 | 5 | 0.662 | 1.207 | 1.82× |
| 1,000 | 10 | 1.210 | 2.664 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
