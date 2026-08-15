# BreadthThrust benchmark (`BreadthThrust` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 121.80M | 0.007 | 143.62M | 8.701 | 1059.80× | 1249.61× |
| 10,000 | 0.055 | 180.29M | 0.059 | 170.79M | 82.507 | 1487.53× | 1409.17× |
| 100,000 | 0.571 | 175.19M | 0.553 | 180.77M | 839.279 | 1470.36× | 1517.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.439 | 5.05× |
| 1 | 5 | 0.325 | 1.469 | 4.53× |
| 1 | 10 | 0.444 | 2.874 | 6.47× |
| 10 | 1 | 0.053 | 0.325 | 6.15× |
| 10 | 5 | 0.189 | 1.621 | 8.59× |
| 10 | 10 | 0.479 | 3.643 | 7.61× |
| 100 | 1 | 0.048 | 1.118 | 23.48× |
| 100 | 5 | 0.247 | 6.116 | 24.80× |
| 100 | 10 | 0.450 | 12.601 | 28.02× |
| 1,000 | 1 | 0.058 | 9.079 | 156.33× |
| 1,000 | 5 | 0.302 | 51.012 | 169.00× |
| 1,000 | 10 | 0.670 | 91.547 | 136.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
