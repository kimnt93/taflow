# RollingMinimumIndex benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.14M | 0.006 | 170.94M | 0.034 | 4.89× | 5.76× |
| 10,000 | 0.053 | 187.97M | 0.050 | 201.15M | 0.094 | 1.76× | 1.88× |
| 100,000 | 0.513 | 194.81M | 0.486 | 205.89M | 0.695 | 1.35× | 1.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.143 | 0.125 | 0.88× |
| 1 | 5 | 0.335 | 0.463 | 1.38× |
| 1 | 10 | 0.512 | 0.945 | 1.85× |
| 10 | 1 | 0.050 | 0.087 | 1.76× |
| 10 | 5 | 0.240 | 0.449 | 1.87× |
| 10 | 10 | 0.475 | 0.892 | 1.88× |
| 100 | 1 | 0.048 | 0.095 | 1.96× |
| 100 | 5 | 0.218 | 0.438 | 2.01× |
| 100 | 10 | 0.483 | 0.964 | 1.99× |
| 1,000 | 1 | 0.054 | 0.093 | 1.70× |
| 1,000 | 5 | 0.233 | 0.480 | 2.05× |
| 1,000 | 10 | 0.540 | 1.059 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
