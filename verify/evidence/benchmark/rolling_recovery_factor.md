# RollingRecoveryFactor benchmark (`rolling recovery factor on equity` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.57M | 0.057 | 17.44M | 0.200 | 3.51× | 3.48× |
| 10,000 | 0.528 | 18.93M | 0.527 | 18.97M | 1.386 | 2.62× | 2.63× |
| 100,000 | 5.202 | 19.22M | 5.408 | 18.49M | 15.780 | 3.03× | 2.92× |
| 1,000,000 | 51.181 | 19.54M | 53.780 | 18.59M | 156.979 | 3.07× | 2.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.148 | 1.57× |
| 1 | 5 | 0.263 | 0.430 | 1.64× |
| 1 | 10 | 0.460 | 0.830 | 1.81× |
| 10 | 1 | 0.051 | 0.085 | 1.66× |
| 10 | 5 | 0.219 | 0.409 | 1.87× |
| 10 | 10 | 0.470 | 0.874 | 1.86× |
| 100 | 1 | 0.058 | 0.196 | 3.41× |
| 100 | 5 | 0.251 | 1.020 | 4.07× |
| 100 | 10 | 0.504 | 1.913 | 3.79× |
| 1,000 | 1 | 0.115 | 0.297 | 2.59× |
| 1,000 | 5 | 0.252 | 1.037 | 4.11× |
| 1,000 | 10 | 0.541 | 2.264 | 4.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
