# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.14M | 0.012 | 85.45M | 0.051 | 4.23× | 4.35× |
| 10,000 | 0.113 | 88.79M | 0.118 | 84.78M | 0.091 | 0.81× | 0.77× |
| 100,000 | 1.070 | 93.45M | 1.043 | 95.88M | 0.666 | 0.62× | 0.64× |
| 1,000,000 | 11.022 | 90.73M | 10.628 | 94.09M | 5.907 | 0.54× | 0.56× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.158 | 1.96× |
| 1 | 5 | 0.307 | 0.495 | 1.62× |
| 1 | 10 | 0.507 | 1.035 | 2.04× |
| 10 | 1 | 0.052 | 0.101 | 1.95× |
| 10 | 5 | 0.233 | 0.502 | 2.16× |
| 10 | 10 | 0.498 | 0.973 | 1.96× |
| 100 | 1 | 0.053 | 0.119 | 2.26× |
| 100 | 5 | 0.343 | 0.595 | 1.73× |
| 100 | 10 | 0.505 | 0.988 | 1.96× |
| 1,000 | 1 | 0.063 | 0.103 | 1.63× |
| 1,000 | 5 | 0.241 | 0.499 | 2.07× |
| 1,000 | 10 | 0.495 | 1.060 | 2.14× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
