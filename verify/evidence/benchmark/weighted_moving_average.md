# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 142.88M | 0.006 | 159.65M | 0.045 | 6.36× | 7.11× |
| 10,000 | 0.043 | 234.73M | 0.038 | 263.09M | 0.070 | 1.63× | 1.83× |
| 100,000 | 0.449 | 222.48M | 0.363 | 275.49M | 0.258 | 0.57× | 0.71× |
| 1,000,000 | 4.299 | 232.61M | 3.716 | 269.07M | 2.146 | 0.50× | 0.58× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.106 | 1.73× |
| 1 | 5 | 0.398 | 0.688 | 1.73× |
| 1 | 10 | 0.495 | 1.029 | 2.08× |
| 10 | 1 | 0.075 | 0.100 | 1.33× |
| 10 | 5 | 0.234 | 0.512 | 2.19× |
| 10 | 10 | 0.501 | 0.959 | 1.92× |
| 100 | 1 | 0.049 | 0.090 | 1.82× |
| 100 | 5 | 0.223 | 0.443 | 1.99× |
| 100 | 10 | 0.475 | 0.990 | 2.08× |
| 1,000 | 1 | 0.058 | 0.102 | 1.75× |
| 1,000 | 5 | 0.234 | 0.452 | 1.93× |
| 1,000 | 10 | 0.480 | 1.046 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
