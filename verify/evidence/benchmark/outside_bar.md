# OutsideBar benchmark (`outside bar relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 337.54M | 0.001 | 685.15M | 0.024 | 8.18× | 16.60× |
| 10,000 | 0.011 | 874.53M | 0.008 | 1.23G | 0.047 | 4.09× | 5.76× |
| 100,000 | 0.098 | 1.02G | 0.071 | 1.41G | 0.267 | 2.71× | 3.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.084 | 1.04× |
| 1 | 5 | 0.247 | 0.353 | 1.43× |
| 1 | 10 | 0.354 | 0.766 | 2.16× |
| 10 | 1 | 0.050 | 0.084 | 1.68× |
| 10 | 5 | 0.197 | 0.372 | 1.89× |
| 10 | 10 | 0.400 | 0.779 | 1.94× |
| 100 | 1 | 0.043 | 0.077 | 1.79× |
| 100 | 5 | 0.182 | 0.408 | 2.24× |
| 100 | 10 | 0.441 | 0.770 | 1.75× |
| 1,000 | 1 | 0.043 | 0.080 | 1.87× |
| 1,000 | 5 | 0.181 | 0.490 | 2.70× |
| 1,000 | 10 | 0.384 | 1.255 | 3.27× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
