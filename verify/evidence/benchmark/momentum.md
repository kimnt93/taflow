# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 206.14M | 0.004 | 268.70M | 0.029 | 5.99× | 7.81× |
| 10,000 | 0.020 | 511.18M | 0.017 | 601.21M | 0.032 | 1.65× | 1.95× |
| 100,000 | 0.167 | 599.18M | 0.148 | 677.00M | 0.064 | 0.38× | 0.43× |
| 1,000,000 | 1.956 | 511.31M | 1.618 | 618.14M | 0.609 | 0.31× | 0.38× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.100 | 0.93× |
| 1 | 5 | 0.279 | 0.479 | 1.72× |
| 1 | 10 | 0.475 | 0.919 | 1.94× |
| 10 | 1 | 0.050 | 0.087 | 1.72× |
| 10 | 5 | 0.227 | 0.439 | 1.93× |
| 10 | 10 | 0.458 | 0.953 | 2.08× |
| 100 | 1 | 0.055 | 0.101 | 1.82× |
| 100 | 5 | 0.214 | 0.432 | 2.02× |
| 100 | 10 | 0.499 | 0.897 | 1.80× |
| 1,000 | 1 | 0.050 | 0.086 | 1.71× |
| 1,000 | 5 | 0.229 | 0.421 | 1.84× |
| 1,000 | 10 | 0.471 | 0.907 | 1.92× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
