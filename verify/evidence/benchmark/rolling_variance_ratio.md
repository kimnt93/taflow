# RollingVarianceRatio benchmark (`VarianceRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.190 | 5.26M | 0.201 | 4.97M | 0.463 | 2.44× | 2.30× |
| 10,000 | 1.922 | 5.20M | 1.906 | 5.25M | 2.601 | 1.35× | 1.36× |
| 100,000 | 18.442 | 5.42M | 18.845 | 5.31M | 22.867 | 1.24× | 1.21× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.155 | 0.286 | 1.85× |
| 1 | 5 | 0.350 | 1.502 | 4.29× |
| 1 | 10 | 0.508 | 2.617 | 5.15× |
| 10 | 1 | 0.052 | 0.255 | 4.87× |
| 10 | 5 | 0.238 | 1.495 | 6.27× |
| 10 | 10 | 0.571 | 2.735 | 4.79× |
| 100 | 1 | 0.066 | 0.282 | 4.26× |
| 100 | 5 | 0.293 | 1.533 | 5.23× |
| 100 | 10 | 0.541 | 2.814 | 5.20× |
| 1,000 | 1 | 0.250 | 0.492 | 1.97× |
| 1,000 | 5 | 0.405 | 2.663 | 6.57× |
| 1,000 | 10 | 0.932 | 5.674 | 6.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
