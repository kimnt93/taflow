# MathAcos benchmark (`ACOS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 124.59M | 0.007 | 141.39M | 0.036 | 4.50× | 5.10× |
| 10,000 | 0.069 | 144.35M | 0.068 | 147.21M | 0.097 | 1.41× | 1.43× |
| 100,000 | 0.755 | 132.44M | 0.667 | 149.83M | 0.681 | 0.90× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.135 | 1.03× |
| 1 | 5 | 0.261 | 0.435 | 1.67× |
| 1 | 10 | 0.389 | 0.959 | 2.47× |
| 10 | 1 | 0.046 | 0.091 | 1.99× |
| 10 | 5 | 0.179 | 0.404 | 2.25× |
| 10 | 10 | 0.383 | 0.849 | 2.22× |
| 100 | 1 | 0.049 | 0.089 | 1.81× |
| 100 | 5 | 0.198 | 0.459 | 2.32× |
| 100 | 10 | 0.413 | 0.909 | 2.20× |
| 1,000 | 1 | 0.053 | 0.094 | 1.78× |
| 1,000 | 5 | 0.207 | 0.493 | 2.38× |
| 1,000 | 10 | 0.495 | 0.993 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
