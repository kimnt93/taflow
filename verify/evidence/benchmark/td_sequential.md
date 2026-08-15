# TomDeMarkSequential benchmark (`TDSequential` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 125.22M | 0.007 | 140.94M | 0.598 | 74.84× | 84.24× |
| 10,000 | 0.071 | 141.70M | 0.070 | 141.96M | 4.028 | 57.08× | 57.19× |
| 100,000 | 0.728 | 137.36M | 0.697 | 143.39M | 45.841 | 62.97× | 65.73× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.356 | 3.31× |
| 1 | 5 | 0.248 | 1.828 | 7.36× |
| 1 | 10 | 0.458 | 3.166 | 6.91× |
| 10 | 1 | 0.049 | 0.277 | 5.62× |
| 10 | 5 | 0.181 | 1.478 | 8.14× |
| 10 | 10 | 0.396 | 3.009 | 7.59× |
| 100 | 1 | 0.044 | 0.303 | 6.81× |
| 100 | 5 | 0.205 | 1.638 | 7.98× |
| 100 | 10 | 0.385 | 3.460 | 8.99× |
| 1,000 | 1 | 0.062 | 0.905 | 14.49× |
| 1,000 | 5 | 0.223 | 4.126 | 18.52× |
| 1,000 | 10 | 0.465 | 7.903 | 17.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
