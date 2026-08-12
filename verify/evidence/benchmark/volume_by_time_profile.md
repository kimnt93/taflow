# VolumeByTimeProfile benchmark (`VolumeByTimeProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.059 | 16.86M | 0.052 | 19.13M | 1.540 | 25.96× | 29.46× |
| 10,000 | 0.485 | 20.64M | 0.427 | 23.42M | 14.554 | 30.04× | 34.09× |
| 100,000 | 5.674 | 17.63M | 4.276 | 23.39M | 184.459 | 32.51× | 43.14× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.338 | 3.95× |
| 1 | 5 | 0.419 | 1.534 | 3.66× |
| 1 | 10 | 0.620 | 2.980 | 4.81× |
| 10 | 1 | 0.076 | 0.306 | 4.03× |
| 10 | 5 | 0.298 | 1.591 | 5.34× |
| 10 | 10 | 0.638 | 3.266 | 5.12× |
| 100 | 1 | 0.067 | 0.427 | 6.35× |
| 100 | 5 | 0.309 | 2.252 | 7.28× |
| 100 | 10 | 0.639 | 4.605 | 7.21× |
| 1,000 | 1 | 0.114 | 1.964 | 17.26× |
| 1,000 | 5 | 0.309 | 9.448 | 30.56× |
| 1,000 | 10 | 0.694 | 19.186 | 27.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
