# RollingMinimum benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 17.84M | 0.045 | 22.45M | 0.034 | 0.60× | 0.75× |
| 10,000 | 0.383 | 26.08M | 0.346 | 28.89M | 0.078 | 0.20× | 0.22× |
| 100,000 | 3.445 | 29.02M | 3.345 | 29.89M | 0.498 | 0.14× | 0.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.138 | 0.110 | 0.80× |
| 1 | 5 | 0.389 | 0.487 | 1.25× |
| 1 | 10 | 0.599 | 0.959 | 1.60× |
| 10 | 1 | 0.063 | 0.091 | 1.45× |
| 10 | 5 | 0.297 | 0.439 | 1.48× |
| 10 | 10 | 0.618 | 0.966 | 1.56× |
| 100 | 1 | 0.069 | 0.097 | 1.41× |
| 100 | 5 | 0.297 | 0.485 | 1.63× |
| 100 | 10 | 0.618 | 0.950 | 1.54× |
| 1,000 | 1 | 0.109 | 0.102 | 0.94× |
| 1,000 | 5 | 0.309 | 0.471 | 1.53× |
| 1,000 | 10 | 0.621 | 1.014 | 1.63× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
