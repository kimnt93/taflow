# MathSinh benchmark (`SINH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.04M | 0.028 | 35.80M | 0.034 | 1.10× | 1.23× |
| 10,000 | 0.218 | 45.83M | 0.213 | 47.03M | 0.087 | 0.40× | 0.41× |
| 100,000 | 2.362 | 42.34M | 2.156 | 46.39M | 0.641 | 0.27× | 0.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.165 | 0.135 | 0.82× |
| 1 | 5 | 0.443 | 0.475 | 1.07× |
| 1 | 10 | 0.546 | 0.876 | 1.60× |
| 10 | 1 | 0.063 | 0.084 | 1.33× |
| 10 | 5 | 0.277 | 0.414 | 1.49× |
| 10 | 10 | 0.563 | 0.868 | 1.54× |
| 100 | 1 | 0.066 | 0.084 | 1.28× |
| 100 | 5 | 0.296 | 0.410 | 1.39× |
| 100 | 10 | 0.613 | 0.899 | 1.47× |
| 1,000 | 1 | 0.101 | 0.096 | 0.95× |
| 1,000 | 5 | 0.278 | 0.464 | 1.67× |
| 1,000 | 10 | 0.591 | 0.933 | 1.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
