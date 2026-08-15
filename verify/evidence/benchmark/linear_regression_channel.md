# LinearRegressionChannel benchmark (`LinRegChannel` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.069 | 14.52M | 0.068 | 14.67M | 0.582 | 8.45× | 8.54× |
| 10,000 | 0.679 | 14.72M | 0.686 | 14.57M | 4.232 | 6.23× | 6.17× |
| 100,000 | 7.008 | 14.27M | 6.758 | 14.80M | 44.674 | 6.37× | 6.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.078 | 0.280 | 3.60× |
| 1 | 5 | 0.257 | 1.403 | 5.45× |
| 1 | 10 | 0.420 | 2.579 | 6.14× |
| 10 | 1 | 0.046 | 0.259 | 5.60× |
| 10 | 5 | 0.194 | 1.448 | 7.46× |
| 10 | 10 | 0.403 | 2.796 | 6.95× |
| 100 | 1 | 0.060 | 0.300 | 5.01× |
| 100 | 5 | 0.217 | 1.613 | 7.43× |
| 100 | 10 | 0.418 | 3.077 | 7.36× |
| 1,000 | 1 | 0.127 | 0.840 | 6.61× |
| 1,000 | 5 | 0.229 | 3.689 | 16.13× |
| 1,000 | 10 | 0.456 | 7.473 | 16.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
