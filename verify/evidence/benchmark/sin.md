# MathSin benchmark (`SIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 87.90M | 0.011 | 94.99M | 0.038 | 3.34× | 3.61× |
| 10,000 | 0.144 | 69.52M | 0.144 | 69.37M | 0.183 | 1.27× | 1.27× |
| 100,000 | 1.544 | 64.78M | 1.482 | 67.48M | 1.546 | 1.00× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.114 | 1.03× |
| 1 | 5 | 0.347 | 0.460 | 1.32× |
| 1 | 10 | 0.402 | 0.898 | 2.24× |
| 10 | 1 | 0.039 | 0.088 | 2.27× |
| 10 | 5 | 0.192 | 0.455 | 2.38× |
| 10 | 10 | 0.444 | 0.962 | 2.17× |
| 100 | 1 | 0.040 | 0.088 | 2.18× |
| 100 | 5 | 0.197 | 0.444 | 2.26× |
| 100 | 10 | 0.410 | 0.957 | 2.33× |
| 1,000 | 1 | 0.061 | 0.110 | 1.81× |
| 1,000 | 5 | 0.211 | 0.499 | 2.36× |
| 1,000 | 10 | 0.452 | 1.052 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
