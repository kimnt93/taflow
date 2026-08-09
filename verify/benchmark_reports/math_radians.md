# MathRadians benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 213.56M | 0.004 | 273.90M | 0.003 | 0.58× | 0.74× |
| 10,000 | 0.016 | 636.00M | 0.013 | 748.75M | 0.015 | 0.95× | 1.12× |
| 100,000 | 0.141 | 707.97M | 0.113 | 888.09M | 0.124 | 0.88× | 1.10× |
| 1,000,000 | 3.057 | 327.13M | 2.778 | 359.98M | 1.368 | 0.45× | 0.49× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.057 | 0.43× |
| 1 | 5 | 0.243 | 0.219 | 0.90× |
| 1 | 10 | 0.516 | 0.430 | 0.83× |
| 10 | 1 | 0.052 | 0.047 | 0.90× |
| 10 | 5 | 0.241 | 0.208 | 0.86× |
| 10 | 10 | 0.464 | 0.404 | 0.87× |
| 100 | 1 | 0.057 | 0.063 | 1.09× |
| 100 | 5 | 0.258 | 0.193 | 0.75× |
| 100 | 10 | 0.537 | 0.410 | 0.76× |
| 1,000 | 1 | 0.052 | 0.046 | 0.87× |
| 1,000 | 5 | 0.234 | 0.215 | 0.92× |
| 1,000 | 10 | 0.518 | 0.479 | 0.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
