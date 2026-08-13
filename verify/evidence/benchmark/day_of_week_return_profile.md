# DayOfWeekReturnProfile benchmark (`DayOfWeekProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.283 | 3.54M | 0.278 | 3.60M | 1.141 | 4.03× | 4.10× |
| 10,000 | 2.550 | 3.92M | 2.642 | 3.79M | 7.167 | 2.81× | 2.71× |
| 100,000 | 26.703 | 3.74M | 26.855 | 3.72M | 78.279 | 2.93× | 2.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.148 | 0.283 | 1.91× |
| 1 | 5 | 0.470 | 1.163 | 2.47× |
| 1 | 10 | 0.711 | 2.337 | 3.29× |
| 10 | 1 | 0.084 | 0.234 | 2.77× |
| 10 | 5 | 0.359 | 1.318 | 3.67× |
| 10 | 10 | 0.700 | 2.470 | 3.53× |
| 100 | 1 | 0.108 | 0.311 | 2.88× |
| 100 | 5 | 0.350 | 1.679 | 4.80× |
| 100 | 10 | 0.709 | 3.140 | 4.43× |
| 1,000 | 1 | 0.361 | 7.197 | 19.94× |
| 1,000 | 5 | 0.677 | 5.472 | 8.08× |
| 1,000 | 10 | 1.036 | 10.562 | 10.19× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
