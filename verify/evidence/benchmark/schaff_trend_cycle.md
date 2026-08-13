# SchaffTrendCycle benchmark (`stc` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.296 | 3.38M | 0.287 | 3.49M | 30.532 | 103.27× | 106.55× |
| 10,000 | 3.156 | 3.17M | 3.152 | 3.17M | 273.843 | 86.76× | 86.89× |
| 100,000 | 27.987 | 3.57M | 27.829 | 3.59M | 2728.139 | 97.48× | 98.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.241 | 2.04× |
| 1 | 5 | 0.446 | 0.968 | 2.17× |
| 1 | 10 | 0.668 | 1.929 | 2.89× |
| 10 | 1 | 0.072 | 0.211 | 2.93× |
| 10 | 5 | 0.330 | 0.943 | 2.86× |
| 10 | 10 | 0.700 | 1.942 | 2.78× |
| 100 | 1 | 0.109 | 4.748 | 43.50× |
| 100 | 5 | 0.427 | 23.875 | 55.88× |
| 100 | 10 | 0.756 | 57.191 | 75.61× |
| 1,000 | 1 | 0.446 | 28.780 | 64.58× |
| 1,000 | 5 | 0.835 | 168.448 | 201.73× |
| 1,000 | 10 | 1.212 | 337.841 | 278.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
