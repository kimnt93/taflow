# EhlersStochastic benchmark (`EhlersStochastic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.036 | 27.83M | 0.034 | 29.69M | 0.190 | 5.30× | 5.65× |
| 10,000 | 0.322 | 31.09M | 0.331 | 30.23M | 0.797 | 2.48× | 2.41× |
| 100,000 | 3.799 | 26.32M | 5.126 | 19.51M | 8.309 | 2.19× | 1.62× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.068 | 0.260 | 3.83× |
| 1 | 5 | 0.321 | 0.999 | 3.11× |
| 1 | 10 | 0.398 | 2.282 | 5.74× |
| 10 | 1 | 0.046 | 0.190 | 4.12× |
| 10 | 5 | 0.186 | 1.011 | 5.43× |
| 10 | 10 | 0.398 | 2.477 | 6.22× |
| 100 | 1 | 0.047 | 0.227 | 4.87× |
| 100 | 5 | 0.205 | 0.984 | 4.79× |
| 100 | 10 | 0.502 | 2.257 | 4.50× |
| 1,000 | 1 | 0.087 | 0.282 | 3.24× |
| 1,000 | 5 | 0.219 | 1.470 | 6.70× |
| 1,000 | 10 | 0.461 | 3.026 | 6.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
