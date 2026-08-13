# InsideBar benchmark (`inside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.048 | 20.70M | 0.044 | 22.79M | 0.022 | 0.45× | 0.49× |
| 10,000 | 0.367 | 27.23M | 0.352 | 28.41M | 0.040 | 0.11× | 0.11× |
| 100,000 | 3.451 | 28.98M | 3.449 | 28.99M | 0.222 | 0.06× | 0.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.102 | 0.75× |
| 1 | 5 | 0.491 | 0.406 | 0.83× |
| 1 | 10 | 0.627 | 0.715 | 1.14× |
| 10 | 1 | 0.066 | 0.081 | 1.21× |
| 10 | 5 | 0.279 | 0.348 | 1.24× |
| 10 | 10 | 0.580 | 0.742 | 1.28× |
| 100 | 1 | 0.065 | 0.076 | 1.18× |
| 100 | 5 | 0.284 | 0.334 | 1.17× |
| 100 | 10 | 0.616 | 0.708 | 1.15× |
| 1,000 | 1 | 0.099 | 0.077 | 0.78× |
| 1,000 | 5 | 0.274 | 0.500 | 1.83× |
| 1,000 | 10 | 0.639 | 1.091 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
