# PlusDirectionalMovement benchmark (`PLUS_DM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.79M | 0.009 | 111.90M | 0.041 | 3.60× | 4.64× |
| 10,000 | 0.062 | 161.07M | 0.065 | 154.95M | 0.100 | 1.62× | 1.56× |
| 100,000 | 0.651 | 153.69M | 0.607 | 164.77M | 0.642 | 0.99× | 1.06× |
| 1,000,000 | 6.942 | 144.05M | 6.142 | 162.82M | 5.924 | 0.85× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.088 | 0.134 | 1.52× |
| 1 | 5 | 0.302 | 0.556 | 1.84× |
| 1 | 10 | 0.576 | 1.294 | 2.25× |
| 10 | 1 | 0.075 | 0.136 | 1.81× |
| 10 | 5 | 0.338 | 0.548 | 1.62× |
| 10 | 10 | 0.617 | 1.291 | 2.09× |
| 100 | 1 | 0.089 | 0.150 | 1.67× |
| 100 | 5 | 0.300 | 0.550 | 1.83× |
| 100 | 10 | 0.563 | 1.320 | 2.35× |
| 1,000 | 1 | 0.085 | 0.127 | 1.49× |
| 1,000 | 5 | 0.315 | 0.590 | 1.87× |
| 1,000 | 10 | 0.596 | 1.312 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
