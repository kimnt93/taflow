# SharkPattern benchmark (`Shark` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 72.87M | 0.012 | 85.07M | 0.212 | 15.45× | 18.04× |
| 10,000 | 0.097 | 103.60M | 0.092 | 108.13M | 1.359 | 14.08× | 14.69× |
| 100,000 | 0.918 | 108.97M | 0.909 | 110.02M | 12.407 | 13.52× | 13.65× |
| 1,000,000 | 10.124 | 98.78M | 8.931 | 111.97M | 125.197 | 12.37× | 14.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.212 | 3.06× |
| 1 | 5 | 0.319 | 1.479 | 4.64× |
| 1 | 10 | 0.540 | 1.668 | 3.09× |
| 10 | 1 | 0.059 | 0.165 | 2.82× |
| 10 | 5 | 0.246 | 1.096 | 4.45× |
| 10 | 10 | 0.522 | 1.715 | 3.29× |
| 100 | 1 | 0.054 | 0.180 | 3.32× |
| 100 | 5 | 0.259 | 1.146 | 4.43× |
| 100 | 10 | 0.551 | 1.796 | 3.26× |
| 1,000 | 1 | 0.070 | 0.300 | 4.32× |
| 1,000 | 5 | 0.257 | 1.824 | 7.10× |
| 1,000 | 10 | 0.545 | 3.165 | 5.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
