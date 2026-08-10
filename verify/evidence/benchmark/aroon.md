# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.037 | 27.05M | 0.030 | 32.93M | 0.041 | 1.11× | 1.35× |
| 10,000 | 0.292 | 34.21M | 0.292 | 34.26M | 0.145 | 0.50× | 0.50× |
| 100,000 | 3.150 | 31.74M | 3.114 | 32.11M | 1.156 | 0.37× | 0.37× |
| 1,000,000 | 29.198 | 34.25M | 28.589 | 34.98M | 12.875 | 0.44× | 0.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.153 | 1.17× |
| 1 | 5 | 0.394 | 0.495 | 1.26× |
| 1 | 10 | 0.509 | 1.019 | 2.00× |
| 10 | 1 | 0.055 | 0.104 | 1.88× |
| 10 | 5 | 0.232 | 0.483 | 2.08× |
| 10 | 10 | 0.486 | 0.997 | 2.05× |
| 100 | 1 | 0.051 | 0.093 | 1.81× |
| 100 | 5 | 0.231 | 0.453 | 1.96× |
| 100 | 10 | 0.485 | 0.953 | 1.96× |
| 1,000 | 1 | 0.082 | 0.111 | 1.36× |
| 1,000 | 5 | 0.240 | 0.541 | 2.25× |
| 1,000 | 10 | 0.538 | 1.042 | 1.94× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
