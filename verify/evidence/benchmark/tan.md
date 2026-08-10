# MathTan benchmark (`TAN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 48.22M | 0.019 | 53.96M | 0.048 | 2.33× | 2.60× |
| 10,000 | 0.214 | 46.70M | 0.215 | 46.52M | 0.237 | 1.11× | 1.10× |
| 100,000 | 2.061 | 48.53M | 2.136 | 46.81M | 1.992 | 0.97× | 0.93× |
| 1,000,000 | 20.351 | 49.14M | 20.704 | 48.30M | 20.465 | 1.01× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.136 | 1.38× |
| 1 | 5 | 0.291 | 0.449 | 1.54× |
| 1 | 10 | 0.444 | 0.944 | 2.13× |
| 10 | 1 | 0.067 | 0.114 | 1.70× |
| 10 | 5 | 0.249 | 0.481 | 1.93× |
| 10 | 10 | 0.501 | 0.854 | 1.71× |
| 100 | 1 | 0.051 | 0.084 | 1.63× |
| 100 | 5 | 0.223 | 0.505 | 2.27× |
| 100 | 10 | 0.547 | 0.990 | 1.81× |
| 1,000 | 1 | 0.079 | 0.116 | 1.47× |
| 1,000 | 5 | 0.272 | 0.557 | 2.05× |
| 1,000 | 10 | 0.585 | 1.168 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
