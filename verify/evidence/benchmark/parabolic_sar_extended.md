# ParabolicSarExtended benchmark (`SAREXT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.055 | 18.07M | 0.044 | 22.85M | 0.050 | 0.90× | 1.13× |
| 10,000 | 0.377 | 26.49M | 0.350 | 28.59M | 0.089 | 0.24× | 0.26× |
| 100,000 | 3.439 | 29.08M | 3.368 | 29.69M | 0.610 | 0.18× | 0.18× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.127 | 0.169 | 1.33× |
| 1 | 5 | 0.458 | 0.562 | 1.23× |
| 1 | 10 | 0.628 | 1.117 | 1.78× |
| 10 | 1 | 0.068 | 0.110 | 1.62× |
| 10 | 5 | 0.311 | 0.526 | 1.69× |
| 10 | 10 | 0.644 | 1.156 | 1.80× |
| 100 | 1 | 0.085 | 0.110 | 1.29× |
| 100 | 5 | 0.311 | 0.546 | 1.76× |
| 100 | 10 | 0.910 | 1.378 | 1.51× |
| 1,000 | 1 | 0.135 | 0.143 | 1.05× |
| 1,000 | 5 | 0.326 | 0.602 | 1.85× |
| 1,000 | 10 | 0.692 | 1.176 | 1.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
