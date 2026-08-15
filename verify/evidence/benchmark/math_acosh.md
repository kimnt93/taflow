# MathAcosh benchmark (`numpy.arccosh` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 91.30M | 0.010 | 100.01M | 0.021 | 1.90× | 2.08× |
| 10,000 | 0.108 | 92.26M | 0.097 | 103.29M | 0.108 | 1.00× | 1.11× |
| 100,000 | 0.990 | 101.04M | 0.995 | 100.50M | 0.988 | 1.00× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.086 | 1.25× |
| 1 | 5 | 0.194 | 0.269 | 1.39× |
| 1 | 10 | 0.376 | 0.557 | 1.48× |
| 10 | 1 | 0.040 | 0.052 | 1.30× |
| 10 | 5 | 0.206 | 0.271 | 1.32× |
| 10 | 10 | 0.378 | 0.575 | 1.52× |
| 100 | 1 | 0.052 | 0.061 | 1.18× |
| 100 | 5 | 0.188 | 0.279 | 1.48× |
| 100 | 10 | 0.384 | 0.584 | 1.52× |
| 1,000 | 1 | 0.052 | 0.064 | 1.23× |
| 1,000 | 5 | 0.183 | 0.349 | 1.91× |
| 1,000 | 10 | 0.401 | 0.717 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
