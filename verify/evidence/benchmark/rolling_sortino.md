# RollingSortino benchmark (`SortinoRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.54M | 0.020 | 49.32M | 0.209 | 9.95× | 10.32× |
| 10,000 | 0.182 | 54.83M | 0.181 | 55.30M | 0.685 | 3.75× | 3.79× |
| 100,000 | 1.783 | 56.09M | 1.787 | 55.95M | 5.643 | 3.17× | 3.16× |
| 1,000,000 | 18.137 | 55.14M | 17.744 | 56.36M | 55.879 | 3.08× | 3.15× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.082 | 0.266 | 3.23× |
| 1 | 5 | 0.321 | 1.313 | 4.09× |
| 1 | 10 | 0.474 | 2.290 | 4.83× |
| 10 | 1 | 0.054 | 0.220 | 4.04× |
| 10 | 5 | 0.218 | 1.252 | 5.73× |
| 10 | 10 | 0.476 | 2.317 | 4.87× |
| 100 | 1 | 0.053 | 0.224 | 4.23× |
| 100 | 5 | 0.230 | 1.284 | 5.58× |
| 100 | 10 | 0.508 | 2.338 | 4.61× |
| 1,000 | 1 | 0.068 | 0.280 | 4.12× |
| 1,000 | 5 | 0.236 | 1.580 | 6.70× |
| 1,000 | 10 | 0.519 | 2.924 | 5.64× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
