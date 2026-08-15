# TimeOfDayReturnProfile benchmark (`TimeOfDayReturnProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.71M | 0.035 | 28.84M | 1.518 | 34.47× | 43.78× |
| 10,000 | 0.376 | 26.58M | 0.292 | 34.22M | 16.152 | 42.94× | 55.28× |
| 100,000 | 4.298 | 23.27M | 2.966 | 33.72M | 193.300 | 44.98× | 65.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.060 | 0.310 | 5.16× |
| 1 | 5 | 0.294 | 1.629 | 5.54× |
| 1 | 10 | 0.432 | 2.784 | 6.44× |
| 10 | 1 | 0.050 | 0.322 | 6.47× |
| 10 | 5 | 0.203 | 2.101 | 10.35× |
| 10 | 10 | 0.469 | 3.372 | 7.18× |
| 100 | 1 | 0.048 | 0.414 | 8.66× |
| 100 | 5 | 0.226 | 2.335 | 10.34× |
| 100 | 10 | 0.449 | 4.237 | 9.45× |
| 1,000 | 1 | 0.087 | 1.986 | 22.89× |
| 1,000 | 5 | 0.226 | 9.401 | 41.59× |
| 1,000 | 10 | 0.610 | 19.137 | 31.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
