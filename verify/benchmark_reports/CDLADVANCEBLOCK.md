# CandleAdvanceBlock benchmark (`CDLADVANCEBLOCK` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.57M | 0.010 | 100.25M | 0.049 | 4.63× | 4.90× |
| 10,000 | 0.075 | 132.82M | 0.073 | 136.55M | 0.210 | 2.79× | 2.87× |
| 100,000 | 0.768 | 130.20M | 0.770 | 129.82M | 1.865 | 2.43× | 2.42× |
| 1,000,000 | 8.392 | 119.16M | 8.074 | 123.85M | 20.868 | 2.49× | 2.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.125 | 0.97× |
| 1 | 5 | 0.366 | 0.518 | 1.41× |
| 1 | 10 | 0.520 | 0.906 | 1.74× |
| 10 | 1 | 0.052 | 0.093 | 1.80× |
| 10 | 5 | 0.232 | 0.423 | 1.82× |
| 10 | 10 | 0.494 | 0.889 | 1.80× |
| 100 | 1 | 0.055 | 0.095 | 1.74× |
| 100 | 5 | 0.263 | 0.455 | 1.73× |
| 100 | 10 | 0.529 | 1.005 | 1.90× |
| 1,000 | 1 | 0.063 | 0.114 | 1.81× |
| 1,000 | 5 | 0.282 | 0.579 | 2.05× |
| 1,000 | 10 | 0.565 | 1.184 | 2.10× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
