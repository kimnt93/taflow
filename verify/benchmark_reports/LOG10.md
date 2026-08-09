# MathLog10 benchmark (`LOG10` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.35M | 0.011 | 90.76M | 0.034 | 2.91× | 3.05× |
| 10,000 | 0.087 | 115.33M | 0.083 | 119.88M | 0.103 | 1.18× | 1.23× |
| 100,000 | 0.850 | 117.65M | 0.815 | 122.71M | 0.792 | 0.93× | 0.97× |
| 1,000,000 | 9.228 | 108.36M | 8.814 | 113.46M | 7.625 | 0.83× | 0.87× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.101 | 0.95× |
| 1 | 5 | 0.292 | 0.497 | 1.70× |
| 1 | 10 | 0.456 | 0.887 | 1.95× |
| 10 | 1 | 0.050 | 0.088 | 1.76× |
| 10 | 5 | 0.246 | 0.417 | 1.69× |
| 10 | 10 | 0.497 | 0.939 | 1.89× |
| 100 | 1 | 0.052 | 0.091 | 1.76× |
| 100 | 5 | 0.223 | 0.424 | 1.90× |
| 100 | 10 | 0.473 | 0.904 | 1.91× |
| 1,000 | 1 | 0.059 | 0.101 | 1.71× |
| 1,000 | 5 | 0.227 | 0.470 | 2.07× |
| 1,000 | 10 | 0.506 | 0.975 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
