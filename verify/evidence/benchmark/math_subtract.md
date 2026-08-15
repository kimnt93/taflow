# MathSubtract benchmark (`SUB` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 371.20M | 0.001 | 895.58M | 0.033 | 12.43× | 29.98× |
| 10,000 | 0.007 | 1.35G | 0.004 | 2.53G | 0.035 | 4.77× | 8.91× |
| 100,000 | 0.062 | 1.62G | 0.036 | 2.74G | 0.067 | 1.09× | 1.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.181 | 0.119 | 0.66× |
| 1 | 5 | 0.263 | 0.485 | 1.84× |
| 1 | 10 | 0.397 | 0.888 | 2.24× |
| 10 | 1 | 0.039 | 0.088 | 2.23× |
| 10 | 5 | 0.185 | 0.433 | 2.35× |
| 10 | 10 | 0.407 | 0.954 | 2.35× |
| 100 | 1 | 0.050 | 0.087 | 1.75× |
| 100 | 5 | 0.187 | 0.422 | 2.25× |
| 100 | 10 | 0.398 | 0.924 | 2.32× |
| 1,000 | 1 | 0.057 | 0.102 | 1.80× |
| 1,000 | 5 | 0.193 | 0.434 | 2.25× |
| 1,000 | 10 | 0.417 | 0.932 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
