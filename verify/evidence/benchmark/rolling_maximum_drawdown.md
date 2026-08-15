# RollingMaximumDrawdown benchmark (`MaxDrawdown` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.040 | 24.89M | 0.041 | 24.45M | 0.259 | 6.44× | 6.32× |
| 10,000 | 0.418 | 23.92M | 0.431 | 23.20M | 1.388 | 3.32× | 3.22× |
| 100,000 | 5.356 | 18.67M | 4.321 | 23.14M | 11.532 | 2.15× | 2.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.287 | 3.10× |
| 1 | 5 | 0.328 | 1.308 | 3.99× |
| 1 | 10 | 0.429 | 2.260 | 5.26× |
| 10 | 1 | 0.056 | 0.194 | 3.46× |
| 10 | 5 | 0.186 | 0.940 | 5.05× |
| 10 | 10 | 0.387 | 2.184 | 5.64× |
| 100 | 1 | 0.053 | 0.206 | 3.85× |
| 100 | 5 | 0.210 | 1.000 | 4.77× |
| 100 | 10 | 0.427 | 2.256 | 5.28× |
| 1,000 | 1 | 0.087 | 0.306 | 3.52× |
| 1,000 | 5 | 0.208 | 1.527 | 7.33× |
| 1,000 | 10 | 0.525 | 3.403 | 6.48× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
