# QuartileBands benchmark (`QuartileBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.073 | 13.71M | 0.073 | 13.75M | 0.695 | 9.53× | 9.55× |
| 10,000 | 0.740 | 13.51M | 0.737 | 13.58M | 5.535 | 7.48× | 7.52× |
| 100,000 | 7.606 | 13.15M | 7.552 | 13.24M | 57.915 | 7.61× | 7.67× |
| 1,000,000 | 74.221 | 13.47M | 87.941 | 11.37M | 624.432 | 8.41× | 7.10× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.291 | 2.90× |
| 1 | 5 | 0.269 | 1.082 | 4.02× |
| 1 | 10 | 0.619 | 2.206 | 3.56× |
| 10 | 1 | 0.049 | 0.217 | 4.42× |
| 10 | 5 | 0.220 | 1.242 | 5.64× |
| 10 | 10 | 0.473 | 2.436 | 5.15× |
| 100 | 1 | 0.058 | 0.305 | 5.22× |
| 100 | 5 | 0.248 | 1.348 | 5.43× |
| 100 | 10 | 0.508 | 2.842 | 5.59× |
| 1,000 | 1 | 0.134 | 0.937 | 7.00× |
| 1,000 | 5 | 0.282 | 4.224 | 14.95× |
| 1,000 | 10 | 0.553 | 8.585 | 15.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
