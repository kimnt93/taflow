# KnowSureThing benchmark (`KST` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.55M | 0.020 | 51.05M | 0.750 | 36.40× | 38.28× |
| 10,000 | 0.182 | 54.96M | 0.166 | 60.33M | 3.970 | 21.82× | 23.95× |
| 100,000 | 1.630 | 61.37M | 1.764 | 56.68M | 40.710 | 24.98× | 23.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.602 | 7.88× |
| 1 | 5 | 0.337 | 2.745 | 8.14× |
| 1 | 10 | 0.570 | 5.601 | 9.83× |
| 10 | 1 | 0.059 | 0.494 | 8.40× |
| 10 | 5 | 0.254 | 2.715 | 10.70× |
| 10 | 10 | 0.570 | 5.656 | 9.93× |
| 100 | 1 | 0.059 | 0.526 | 8.97× |
| 100 | 5 | 0.260 | 3.020 | 11.60× |
| 100 | 10 | 0.506 | 5.806 | 11.47× |
| 1,000 | 1 | 0.073 | 1.045 | 14.36× |
| 1,000 | 5 | 0.260 | 4.892 | 18.82× |
| 1,000 | 10 | 0.585 | 9.587 | 16.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
