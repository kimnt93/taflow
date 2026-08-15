# RollingWinsorize benchmark (`rolling winsorize` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.83M | 0.048 | 20.97M | 0.575 | 13.13× | 12.06× |
| 10,000 | 0.499 | 20.05M | 0.509 | 19.64M | 3.150 | 6.32× | 6.19× |
| 100,000 | 4.932 | 20.28M | 5.189 | 19.27M | 31.167 | 6.32× | 6.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.058 | 0.437 | 7.52× |
| 1 | 5 | 0.240 | 1.647 | 6.85× |
| 1 | 10 | 0.431 | 3.426 | 7.95× |
| 10 | 1 | 0.045 | 0.300 | 6.69× |
| 10 | 5 | 0.186 | 1.627 | 8.77× |
| 10 | 10 | 0.434 | 3.338 | 7.68× |
| 100 | 1 | 0.054 | 0.386 | 7.14× |
| 100 | 5 | 0.210 | 2.051 | 9.75× |
| 100 | 10 | 0.413 | 4.005 | 9.69× |
| 1,000 | 1 | 0.103 | 0.638 | 6.17× |
| 1,000 | 5 | 0.206 | 2.387 | 11.57× |
| 1,000 | 10 | 0.470 | 4.869 | 10.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
