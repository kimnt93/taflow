# AccelerationBands benchmark (`ACCBANDS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 51.79M | 0.016 | 64.31M | 0.054 | 2.80× | 3.48× |
| 10,000 | 0.108 | 92.84M | 0.096 | 104.24M | 0.123 | 1.14× | 1.29× |
| 100,000 | 1.506 | 66.39M | 1.475 | 67.79M | 1.277 | 0.85× | 0.87× |
| 1,000,000 | 24.292 | 41.17M | 23.724 | 42.15M | 17.421 | 0.72× | 0.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.910 | 6.14× |
| 1 | 5 | 0.605 | 1.604 | 2.65× |
| 1 | 10 | 0.911 | 3.626 | 3.98× |
| 10 | 1 | 0.730 | 0.169 | 0.23× |
| 10 | 5 | 0.331 | 0.637 | 1.93× |
| 10 | 10 | 0.657 | 1.180 | 1.80× |
| 100 | 1 | 0.069 | 0.124 | 1.81× |
| 100 | 5 | 0.367 | 0.673 | 1.84× |
| 100 | 10 | 0.712 | 1.287 | 1.81× |
| 1,000 | 1 | 0.075 | 0.116 | 1.56× |
| 1,000 | 5 | 0.306 | 0.726 | 2.37× |
| 1,000 | 10 | 0.810 | 1.314 | 1.62× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
