# TimeOfDayReturnProfile benchmark (`TimeOfDayReturnProfile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.27M | 0.034 | 29.28M | 1.621 | 36.11× | 47.48× |
| 10,000 | 0.420 | 23.83M | 0.335 | 29.83M | 17.294 | 41.21× | 51.58× |
| 100,000 | 4.379 | 22.84M | 3.132 | 31.93M | 192.989 | 44.07× | 61.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.421 | 3.58× |
| 1 | 5 | 0.214 | 1.589 | 7.43× |
| 1 | 10 | 0.443 | 2.966 | 6.70× |
| 10 | 1 | 0.048 | 0.263 | 5.49× |
| 10 | 5 | 0.195 | 1.465 | 7.51× |
| 10 | 10 | 0.431 | 2.979 | 6.91× |
| 100 | 1 | 0.050 | 0.415 | 8.35× |
| 100 | 5 | 0.214 | 2.272 | 10.61× |
| 100 | 10 | 0.417 | 4.352 | 10.45× |
| 1,000 | 1 | 0.078 | 1.958 | 25.14× |
| 1,000 | 5 | 0.221 | 9.751 | 44.14× |
| 1,000 | 10 | 0.524 | 19.074 | 36.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
