# RollingMode benchmark (`rolling mode` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.086 | 11.57M | 0.078 | 12.86M | 0.043 | 0.50× | 0.55× |
| 10,000 | 0.754 | 13.26M | 0.752 | 13.29M | 0.107 | 0.14× | 0.14× |
| 100,000 | 7.884 | 12.68M | 7.391 | 13.53M | 0.953 | 0.12× | 0.13× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.095 | 1.00× |
| 1 | 5 | 0.283 | 0.411 | 1.45× |
| 1 | 10 | 0.397 | 0.791 | 1.99× |
| 10 | 1 | 0.048 | 0.080 | 1.65× |
| 10 | 5 | 0.178 | 0.372 | 2.09× |
| 10 | 10 | 0.400 | 0.815 | 2.04× |
| 100 | 1 | 0.052 | 0.112 | 2.15× |
| 100 | 5 | 0.202 | 0.527 | 2.60× |
| 100 | 10 | 0.412 | 1.040 | 2.52× |
| 1,000 | 1 | 0.125 | 0.113 | 0.91× |
| 1,000 | 5 | 0.295 | 0.621 | 2.11× |
| 1,000 | 10 | 0.484 | 1.505 | 3.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
