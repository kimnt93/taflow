# OpeningRange benchmark (`anchored opening range` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 69.75M | 0.011 | 92.29M | 0.528 | 36.80× | 48.69× |
| 10,000 | 0.070 | 142.25M | 0.059 | 170.05M | 5.184 | 73.75× | 88.16× |
| 100,000 | 0.647 | 154.47M | 0.493 | 202.88M | 48.578 | 75.04× | 98.56× |
| 1,000,000 | 7.363 | 135.82M | 5.380 | 185.87M | 492.490 | 66.89× | 91.54× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.155 | 1.66× |
| 1 | 5 | 0.354 | 0.541 | 1.53× |
| 1 | 10 | 0.526 | 0.921 | 1.75× |
| 10 | 1 | 0.054 | 0.098 | 1.82× |
| 10 | 5 | 0.228 | 0.492 | 2.16× |
| 10 | 10 | 0.528 | 1.019 | 1.93× |
| 100 | 1 | 0.051 | 0.149 | 2.89× |
| 100 | 5 | 0.237 | 0.704 | 2.97× |
| 100 | 10 | 0.465 | 1.473 | 3.17× |
| 1,000 | 1 | 0.064 | 0.613 | 9.58× |
| 1,000 | 5 | 0.267 | 3.068 | 11.51× |
| 1,000 | 10 | 0.563 | 6.194 | 11.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
