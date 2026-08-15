# AbsoluteBreadthIndex benchmark (`AbsoluteBreadthIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 215.38M | 0.003 | 314.67M | 8.682 | 1869.90× | 2731.95× |
| 10,000 | 0.028 | 355.92M | 0.024 | 417.52M | 83.329 | 2965.85× | 3479.15× |
| 100,000 | 0.243 | 411.04M | 0.226 | 442.88M | 834.298 | 3429.28× | 3694.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.135 | 0.315 | 2.33× |
| 1 | 5 | 0.287 | 1.098 | 3.83× |
| 1 | 10 | 0.407 | 2.118 | 5.20× |
| 10 | 1 | 0.046 | 0.288 | 6.20× |
| 10 | 5 | 0.192 | 1.744 | 9.07× |
| 10 | 10 | 0.387 | 2.972 | 7.68× |
| 100 | 1 | 0.043 | 1.051 | 24.18× |
| 100 | 5 | 0.192 | 5.699 | 29.62× |
| 100 | 10 | 0.397 | 11.079 | 27.93× |
| 1,000 | 1 | 0.051 | 8.640 | 168.79× |
| 1,000 | 5 | 0.297 | 44.094 | 148.48× |
| 1,000 | 10 | 0.453 | 94.934 | 209.41× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
