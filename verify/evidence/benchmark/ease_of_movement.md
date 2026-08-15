# EaseOfMovement benchmark (`EaseOfMovement` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 100.61M | 0.008 | 125.50M | 0.278 | 28.00× | 34.93× |
| 10,000 | 0.074 | 135.43M | 0.069 | 144.45M | 1.235 | 16.72× | 17.84× |
| 100,000 | 0.699 | 143.02M | 0.679 | 147.36M | 10.604 | 15.17× | 15.63× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.101 | 0.294 | 2.90× |
| 1 | 5 | 0.415 | 1.538 | 3.71× |
| 1 | 10 | 0.386 | 2.639 | 6.83× |
| 10 | 1 | 0.049 | 0.269 | 5.45× |
| 10 | 5 | 0.187 | 1.519 | 8.11× |
| 10 | 10 | 0.401 | 2.897 | 7.22× |
| 100 | 1 | 0.051 | 0.270 | 5.33× |
| 100 | 5 | 0.201 | 1.555 | 7.72× |
| 100 | 10 | 0.427 | 2.771 | 6.49× |
| 1,000 | 1 | 0.049 | 0.360 | 7.30× |
| 1,000 | 5 | 0.201 | 2.067 | 10.26× |
| 1,000 | 10 | 0.420 | 4.119 | 9.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
