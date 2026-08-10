# CandleBeltHold benchmark (`CDLBELTHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.82M | 0.018 | 56.04M | 0.042 | 1.78× | 2.38× |
| 10,000 | 0.197 | 50.79M | 0.169 | 59.24M | 0.147 | 0.75× | 0.87× |
| 100,000 | 1.958 | 51.07M | 1.704 | 58.69M | 1.180 | 0.60× | 0.69× |
| 1,000,000 | 19.999 | 50.00M | 16.956 | 58.97M | 11.540 | 0.58× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.136 | 1.09× |
| 1 | 5 | 0.285 | 0.506 | 1.78× |
| 1 | 10 | 0.561 | 0.943 | 1.68× |
| 10 | 1 | 0.063 | 0.100 | 1.59× |
| 10 | 5 | 0.298 | 0.506 | 1.70× |
| 10 | 10 | 0.589 | 0.949 | 1.61× |
| 100 | 1 | 0.060 | 0.105 | 1.75× |
| 100 | 5 | 0.289 | 0.463 | 1.60× |
| 100 | 10 | 0.586 | 1.152 | 1.97× |
| 1,000 | 1 | 0.079 | 0.120 | 1.51× |
| 1,000 | 5 | 0.366 | 0.644 | 1.76× |
| 1,000 | 10 | 0.645 | 1.183 | 1.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
