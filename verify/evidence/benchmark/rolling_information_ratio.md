# RollingInformationRatio benchmark (`InformationRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.035 | 28.61M | 0.034 | 29.63M | 0.267 | 7.65× | 7.93× |
| 10,000 | 0.311 | 32.19M | 0.326 | 30.64M | 0.801 | 2.58× | 2.45× |
| 100,000 | 3.420 | 29.24M | 3.168 | 31.56M | 6.670 | 1.95× | 2.11× |
| 1,000,000 | 32.892 | 30.40M | 37.947 | 26.35M | 69.531 | 2.11× | 1.83× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.273 | 2.34× |
| 1 | 5 | 0.363 | 1.027 | 2.83× |
| 1 | 10 | 0.498 | 2.284 | 4.59× |
| 10 | 1 | 0.055 | 0.198 | 3.60× |
| 10 | 5 | 0.246 | 1.020 | 4.14× |
| 10 | 10 | 0.545 | 2.446 | 4.49× |
| 100 | 1 | 0.062 | 0.209 | 3.36× |
| 100 | 5 | 0.238 | 1.024 | 4.31× |
| 100 | 10 | 0.530 | 2.332 | 4.40× |
| 1,000 | 1 | 0.086 | 0.274 | 3.19× |
| 1,000 | 5 | 0.248 | 1.352 | 5.46× |
| 1,000 | 10 | 0.542 | 3.076 | 5.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
