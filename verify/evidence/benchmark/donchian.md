# Donchian benchmark (`Donchian` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.61M | 0.009 | 113.56M | 0.554 | 51.89× | 62.95× |
| 10,000 | 0.086 | 116.45M | 0.091 | 110.05M | 4.014 | 46.75× | 44.18× |
| 100,000 | 0.792 | 126.21M | 0.723 | 138.24M | 43.999 | 55.53× | 60.83× |
| 1,000,000 | 22.024 | 45.41M | 8.903 | 112.32M | 499.414 | 22.68× | 56.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.074 | 0.288 | 3.86× |
| 1 | 5 | 0.280 | 1.140 | 4.07× |
| 1 | 10 | 0.514 | 2.497 | 4.86× |
| 10 | 1 | 0.051 | 0.244 | 4.78× |
| 10 | 5 | 0.239 | 1.441 | 6.04× |
| 10 | 10 | 0.560 | 2.568 | 4.59× |
| 100 | 1 | 0.059 | 0.278 | 4.68× |
| 100 | 5 | 0.244 | 1.633 | 6.69× |
| 100 | 10 | 0.521 | 2.903 | 5.58× |
| 1,000 | 1 | 0.060 | 0.895 | 14.84× |
| 1,000 | 5 | 0.269 | 3.976 | 14.79× |
| 1,000 | 10 | 1.182 | 8.075 | 6.83× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
