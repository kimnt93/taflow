# DemandIndex benchmark (`DemandIndex` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 68.82M | 0.012 | 84.79M | 0.292 | 20.07× | 24.73× |
| 10,000 | 0.069 | 144.37M | 0.064 | 155.76M | 1.512 | 21.83× | 23.55× |
| 100,000 | 0.643 | 155.59M | 0.531 | 188.21M | 13.247 | 20.61× | 24.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.279 | 3.95× |
| 1 | 5 | 0.469 | 1.216 | 2.60× |
| 1 | 10 | 0.537 | 2.556 | 4.76× |
| 10 | 1 | 0.067 | 0.234 | 3.48× |
| 10 | 5 | 0.268 | 1.106 | 4.13× |
| 10 | 10 | 0.551 | 2.406 | 4.36× |
| 100 | 1 | 0.058 | 0.237 | 4.08× |
| 100 | 5 | 0.260 | 1.351 | 5.20× |
| 100 | 10 | 0.517 | 2.653 | 5.13× |
| 1,000 | 1 | 0.064 | 0.357 | 5.61× |
| 1,000 | 5 | 0.273 | 2.193 | 8.03× |
| 1,000 | 10 | 0.601 | 3.873 | 6.44× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
