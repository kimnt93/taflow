# PositionHold benchmark (`nonzero position hold` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 189.34M | 0.004 | 222.70M | 0.125 | 23.58× | 27.73× |
| 10,000 | 0.024 | 413.67M | 0.023 | 444.01M | 1.153 | 47.71× | 51.21× |
| 100,000 | 0.216 | 461.94M | 0.193 | 519.03M | 11.118 | 51.36× | 57.71× |
| 1,000,000 | 2.395 | 417.51M | 1.980 | 505.12M | 113.834 | 47.53× | 57.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.078 | 0.96× |
| 1 | 5 | 0.229 | 0.299 | 1.31× |
| 1 | 10 | 0.458 | 0.596 | 1.30× |
| 10 | 1 | 0.048 | 0.061 | 1.29× |
| 10 | 5 | 0.207 | 0.287 | 1.38× |
| 10 | 10 | 0.455 | 0.612 | 1.35× |
| 100 | 1 | 0.045 | 0.067 | 1.50× |
| 100 | 5 | 0.211 | 0.337 | 1.60× |
| 100 | 10 | 0.459 | 0.705 | 1.54× |
| 1,000 | 1 | 0.048 | 0.182 | 3.82× |
| 1,000 | 5 | 0.219 | 0.894 | 4.08× |
| 1,000 | 10 | 0.482 | 1.824 | 3.78× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
