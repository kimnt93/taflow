# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.068 | 14.72M | 0.067 | 14.94M | 0.094 | 1.39× | 1.41× |
| 10,000 | 0.696 | 14.36M | 0.627 | 15.95M | 0.691 | 0.99× | 1.10× |
| 100,000 | 6.300 | 15.87M | 6.011 | 16.64M | 5.847 | 0.93× | 0.97× |
| 1,000,000 | 63.273 | 15.80M | 58.985 | 16.95M | 57.174 | 0.90× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.144 | 1.73× |
| 1 | 5 | 0.286 | 0.645 | 2.26× |
| 1 | 10 | 0.490 | 1.029 | 2.10× |
| 10 | 1 | 0.052 | 0.112 | 2.13× |
| 10 | 5 | 0.249 | 0.528 | 2.12× |
| 10 | 10 | 0.490 | 1.022 | 2.09× |
| 100 | 1 | 0.059 | 0.101 | 1.71× |
| 100 | 5 | 0.238 | 0.627 | 2.63× |
| 100 | 10 | 0.564 | 1.114 | 1.98× |
| 1,000 | 1 | 0.103 | 0.180 | 1.75× |
| 1,000 | 5 | 0.293 | 0.855 | 2.92× |
| 1,000 | 10 | 0.586 | 1.666 | 2.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
