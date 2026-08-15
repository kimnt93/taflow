# PivotPoints benchmark (`anchored classic pivot points` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.21M | 0.010 | 95.96M | 0.973 | 65.43× | 93.41× |
| 10,000 | 0.169 | 59.24M | 0.100 | 100.46M | 8.504 | 50.37× | 85.43× |
| 100,000 | 1.063 | 94.08M | 0.778 | 128.51M | 86.401 | 81.28× | 111.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.115 | 1.07× |
| 1 | 5 | 0.314 | 0.381 | 1.21× |
| 1 | 10 | 0.385 | 0.704 | 1.83× |
| 10 | 1 | 0.045 | 0.087 | 1.92× |
| 10 | 5 | 0.183 | 0.409 | 2.24× |
| 10 | 10 | 0.409 | 0.842 | 2.06× |
| 100 | 1 | 0.045 | 0.176 | 3.94× |
| 100 | 5 | 0.199 | 0.826 | 4.15× |
| 100 | 10 | 0.412 | 1.674 | 4.06× |
| 1,000 | 1 | 0.057 | 0.996 | 17.44× |
| 1,000 | 5 | 0.257 | 5.027 | 19.60× |
| 1,000 | 10 | 0.516 | 10.312 | 19.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
