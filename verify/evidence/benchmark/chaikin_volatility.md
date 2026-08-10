# ChaikinVolatility benchmark (`ChaikinVolatility` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 85.55M | 0.011 | 90.25M | 0.252 | 21.53× | 22.71× |
| 10,000 | 0.076 | 131.22M | 0.074 | 135.17M | 0.825 | 10.83× | 11.16× |
| 100,000 | 0.704 | 142.07M | 0.689 | 145.13M | 6.763 | 9.61× | 9.81× |
| 1,000,000 | 7.155 | 139.76M | 6.826 | 146.51M | 65.595 | 9.17× | 9.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.091 | 0.303 | 3.34× |
| 1 | 5 | 0.396 | 1.716 | 4.33× |
| 1 | 10 | 0.567 | 2.955 | 5.21× |
| 10 | 1 | 0.055 | 0.257 | 4.70× |
| 10 | 5 | 0.240 | 1.504 | 6.28× |
| 10 | 10 | 0.502 | 2.655 | 5.29× |
| 100 | 1 | 0.070 | 0.295 | 4.23× |
| 100 | 5 | 0.246 | 1.538 | 6.25× |
| 100 | 10 | 0.515 | 2.957 | 5.74× |
| 1,000 | 1 | 0.066 | 0.322 | 4.86× |
| 1,000 | 5 | 0.243 | 2.003 | 8.23× |
| 1,000 | 10 | 0.606 | 3.455 | 5.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
