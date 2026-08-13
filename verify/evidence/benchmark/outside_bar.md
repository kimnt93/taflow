# OutsideBar benchmark (`outside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.050 | 20.20M | 0.044 | 22.62M | 0.021 | 0.43× | 0.48× |
| 10,000 | 0.353 | 28.33M | 0.350 | 28.60M | 0.041 | 0.12× | 0.12× |
| 100,000 | 3.675 | 27.21M | 3.639 | 27.48M | 0.249 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.203 | 0.139 | 0.68× |
| 1 | 5 | 0.519 | 0.366 | 0.70× |
| 1 | 10 | 0.587 | 0.720 | 1.23× |
| 10 | 1 | 0.064 | 0.071 | 1.11× |
| 10 | 5 | 0.279 | 0.347 | 1.24× |
| 10 | 10 | 0.609 | 0.762 | 1.25× |
| 100 | 1 | 0.066 | 0.070 | 1.06× |
| 100 | 5 | 0.292 | 0.352 | 1.21× |
| 100 | 10 | 0.598 | 0.740 | 1.24× |
| 1,000 | 1 | 0.103 | 0.089 | 0.87× |
| 1,000 | 5 | 0.289 | 0.458 | 1.58× |
| 1,000 | 10 | 0.675 | 1.139 | 1.69× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
