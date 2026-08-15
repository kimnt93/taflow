# Momentum benchmark (`MOM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 458.50M | 0.001 | 931.05M | 0.032 | 14.82× | 30.09× |
| 10,000 | 0.006 | 1.77G | 0.003 | 3.00G | 0.034 | 6.05× | 10.26× |
| 100,000 | 0.055 | 1.82G | 0.030 | 3.33G | 0.062 | 1.13× | 2.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.163 | 1.24× |
| 1 | 5 | 0.314 | 0.484 | 1.54× |
| 1 | 10 | 0.434 | 0.942 | 2.17× |
| 10 | 1 | 0.042 | 0.090 | 2.14× |
| 10 | 5 | 0.179 | 0.426 | 2.39× |
| 10 | 10 | 0.403 | 0.961 | 2.39× |
| 100 | 1 | 0.050 | 0.113 | 2.26× |
| 100 | 5 | 0.207 | 0.473 | 2.29× |
| 100 | 10 | 0.411 | 0.926 | 2.25× |
| 1,000 | 1 | 0.042 | 0.094 | 2.23× |
| 1,000 | 5 | 0.217 | 0.457 | 2.10× |
| 1,000 | 10 | 0.400 | 0.917 | 2.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
