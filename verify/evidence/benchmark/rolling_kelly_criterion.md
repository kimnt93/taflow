# RollingKellyCriterion benchmark (`KellyCriterion` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.46M | 0.022 | 45.77M | 0.190 | 8.66× | 8.71× |
| 10,000 | 0.201 | 49.67M | 0.193 | 51.71M | 0.725 | 3.60× | 3.75× |
| 100,000 | 1.952 | 51.24M | 1.881 | 53.17M | 6.269 | 3.21× | 3.33× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.087 | 0.255 | 2.94× |
| 1 | 5 | 0.276 | 1.091 | 3.96× |
| 1 | 10 | 0.607 | 2.475 | 4.08× |
| 10 | 1 | 0.055 | 0.197 | 3.57× |
| 10 | 5 | 0.272 | 1.174 | 4.31× |
| 10 | 10 | 0.547 | 2.512 | 4.59× |
| 100 | 1 | 0.059 | 0.221 | 3.76× |
| 100 | 5 | 0.312 | 1.189 | 3.81× |
| 100 | 10 | 0.587 | 2.685 | 4.57× |
| 1,000 | 1 | 0.076 | 0.283 | 3.71× |
| 1,000 | 5 | 0.280 | 1.462 | 5.21× |
| 1,000 | 10 | 0.615 | 3.094 | 5.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
