# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 94.51M | 0.011 | 91.25M | 0.055 | 5.16× | 4.98× |
| 10,000 | 0.057 | 175.71M | 0.052 | 191.34M | 0.094 | 1.65× | 1.80× |
| 100,000 | 0.519 | 192.81M | 0.492 | 203.12M | 0.590 | 1.14× | 1.20× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.071 | 0.162 | 2.28× |
| 1 | 5 | 0.327 | 0.485 | 1.48× |
| 1 | 10 | 0.476 | 0.968 | 2.03× |
| 10 | 1 | 0.063 | 0.118 | 1.87× |
| 10 | 5 | 0.294 | 0.508 | 1.73× |
| 10 | 10 | 0.507 | 0.960 | 1.89× |
| 100 | 1 | 0.050 | 0.094 | 1.89× |
| 100 | 5 | 0.245 | 0.541 | 2.21× |
| 100 | 10 | 0.539 | 0.997 | 1.85× |
| 1,000 | 1 | 0.058 | 0.102 | 1.75× |
| 1,000 | 5 | 0.250 | 0.509 | 2.03× |
| 1,000 | 10 | 0.556 | 1.095 | 1.97× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
