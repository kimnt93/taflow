# ParabolicMovingAverageStop benchmark (`pmax` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.019 | 52.40M | 0.017 | 57.45M | 2.992 | 156.79× | 171.89× |
| 10,000 | 0.175 | 57.18M | 0.170 | 58.93M | 16.666 | 95.29× | 98.22× |
| 100,000 | 1.666 | 60.02M | 1.581 | 63.25M | 155.090 | 93.08× | 98.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.282 | 2.48× |
| 1 | 5 | 0.284 | 1.181 | 4.15× |
| 1 | 10 | 0.401 | 2.269 | 5.66× |
| 10 | 1 | 0.055 | 1.939 | 35.08× |
| 10 | 5 | 0.201 | 8.441 | 41.91× |
| 10 | 10 | 0.454 | 17.654 | 38.85× |
| 100 | 1 | 0.052 | 1.870 | 35.62× |
| 100 | 5 | 0.306 | 9.848 | 32.21× |
| 100 | 10 | 0.542 | 20.076 | 37.03× |
| 1,000 | 1 | 0.069 | 3.453 | 50.28× |
| 1,000 | 5 | 0.252 | 17.654 | 70.00× |
| 1,000 | 10 | 0.528 | 35.168 | 66.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
