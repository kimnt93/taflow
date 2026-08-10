# RollingMaximumDrawdown benchmark (`MaxDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.46M | 0.050 | 20.18M | 0.137 | 2.79× | 2.76× |
| 10,000 | 0.482 | 20.75M | 0.501 | 19.97M | 1.247 | 2.59× | 2.49× |
| 100,000 | 4.769 | 20.97M | 4.757 | 21.02M | 12.060 | 2.53× | 2.54× |
| 1,000,000 | 48.328 | 20.69M | 49.518 | 20.19M | 119.397 | 2.47× | 2.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.139 | 0.103 | 0.74× |
| 1 | 5 | 0.348 | 0.416 | 1.19× |
| 1 | 10 | 0.490 | 0.670 | 1.37× |
| 10 | 1 | 0.048 | 0.069 | 1.45× |
| 10 | 5 | 0.223 | 0.315 | 1.41× |
| 10 | 10 | 0.491 | 0.801 | 1.63× |
| 100 | 1 | 0.062 | 0.084 | 1.36× |
| 100 | 5 | 0.232 | 0.385 | 1.66× |
| 100 | 10 | 0.479 | 0.794 | 1.66× |
| 1,000 | 1 | 0.106 | 0.203 | 1.91× |
| 1,000 | 5 | 0.308 | 0.980 | 3.18× |
| 1,000 | 10 | 0.526 | 1.968 | 3.74× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
