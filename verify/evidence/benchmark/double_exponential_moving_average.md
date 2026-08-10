# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.41M | 0.009 | 116.64M | 0.039 | 4.18× | 4.54× |
| 10,000 | 0.063 | 157.82M | 0.065 | 153.12M | 0.102 | 1.61× | 1.56× |
| 100,000 | 0.660 | 151.56M | 0.624 | 160.19M | 0.712 | 1.08× | 1.14× |
| 1,000,000 | 6.898 | 144.98M | 6.205 | 161.17M | 6.903 | 1.00× | 1.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.159 | 1.44× |
| 1 | 5 | 0.370 | 0.473 | 1.28× |
| 1 | 10 | 0.448 | 0.946 | 2.11× |
| 10 | 1 | 0.054 | 0.094 | 1.75× |
| 10 | 5 | 0.247 | 0.473 | 1.92× |
| 10 | 10 | 0.460 | 0.939 | 2.04× |
| 100 | 1 | 0.050 | 0.093 | 1.83× |
| 100 | 5 | 0.223 | 0.465 | 2.08× |
| 100 | 10 | 0.564 | 0.966 | 1.71× |
| 1,000 | 1 | 0.062 | 0.101 | 1.64× |
| 1,000 | 5 | 0.229 | 0.474 | 2.07× |
| 1,000 | 10 | 0.496 | 1.103 | 2.23× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
