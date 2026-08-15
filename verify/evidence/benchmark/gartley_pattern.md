# GartleyPattern benchmark (`Gartley` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.87M | 0.008 | 124.97M | 0.230 | 21.85× | 28.78× |
| 10,000 | 0.098 | 101.80M | 0.090 | 110.61M | 1.371 | 13.95× | 15.16× |
| 100,000 | 0.944 | 105.92M | 0.908 | 110.12M | 12.773 | 13.53× | 14.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.243 | 3.41× |
| 1 | 5 | 0.352 | 0.854 | 2.42× |
| 1 | 10 | 0.405 | 1.632 | 4.03× |
| 10 | 1 | 0.044 | 0.167 | 3.81× |
| 10 | 5 | 0.191 | 1.160 | 6.07× |
| 10 | 10 | 0.418 | 1.646 | 3.94× |
| 100 | 1 | 0.046 | 0.172 | 3.69× |
| 100 | 5 | 0.214 | 1.169 | 5.45× |
| 100 | 10 | 0.402 | 1.780 | 4.42× |
| 1,000 | 1 | 0.056 | 0.349 | 6.24× |
| 1,000 | 5 | 0.221 | 1.749 | 7.91× |
| 1,000 | 10 | 0.397 | 3.007 | 7.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
