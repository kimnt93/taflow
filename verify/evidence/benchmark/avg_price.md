# AveragePrice benchmark (`AVGPRICE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 106.69M | 0.007 | 147.73M | 0.030 | 3.21× | 4.45× |
| 10,000 | 0.032 | 314.72M | 0.029 | 346.22M | 0.039 | 1.23× | 1.35× |
| 100,000 | 0.253 | 395.84M | 0.245 | 407.35M | 0.092 | 0.36× | 0.37× |
| 1,000,000 | 3.710 | 269.53M | 2.939 | 340.28M | 1.912 | 0.52× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.104 | 0.80× |
| 1 | 5 | 0.275 | 0.431 | 1.57× |
| 1 | 10 | 0.478 | 0.884 | 1.85× |
| 10 | 1 | 0.053 | 0.086 | 1.64× |
| 10 | 5 | 0.249 | 0.495 | 1.98× |
| 10 | 10 | 0.532 | 0.956 | 1.80× |
| 100 | 1 | 0.052 | 0.084 | 1.61× |
| 100 | 5 | 0.252 | 0.450 | 1.78× |
| 100 | 10 | 0.562 | 0.977 | 1.74× |
| 1,000 | 1 | 0.060 | 0.098 | 1.62× |
| 1,000 | 5 | 0.257 | 0.455 | 1.77× |
| 1,000 | 10 | 0.560 | 1.015 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
