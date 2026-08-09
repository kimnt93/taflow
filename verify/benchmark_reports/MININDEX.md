# RollingArgmin benchmark (`MININDEX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 136.68M | 0.006 | 156.62M | 0.037 | 5.10× | 5.84× |
| 10,000 | 0.054 | 183.87M | 0.052 | 192.75M | 0.101 | 1.85× | 1.94× |
| 100,000 | 0.535 | 186.83M | 0.515 | 194.20M | 0.695 | 1.30× | 1.35× |
| 1,000,000 | 5.860 | 170.64M | 5.372 | 186.17M | 6.971 | 1.19× | 1.30× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.130 | 0.159 | 1.22× |
| 1 | 5 | 0.250 | 0.485 | 1.94× |
| 1 | 10 | 0.483 | 0.962 | 1.99× |
| 10 | 1 | 0.058 | 0.110 | 1.89× |
| 10 | 5 | 0.231 | 0.455 | 1.97× |
| 10 | 10 | 0.512 | 0.921 | 1.80× |
| 100 | 1 | 0.050 | 0.092 | 1.82× |
| 100 | 5 | 0.250 | 0.490 | 1.96× |
| 100 | 10 | 0.508 | 0.937 | 1.84× |
| 1,000 | 1 | 0.055 | 0.103 | 1.88× |
| 1,000 | 5 | 0.238 | 0.503 | 2.11× |
| 1,000 | 10 | 0.606 | 1.064 | 1.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
