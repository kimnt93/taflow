# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 65.74M | 0.013 | 77.64M | 0.042 | 2.74× | 3.24× |
| 10,000 | 0.095 | 105.01M | 0.086 | 116.12M | 0.127 | 1.34× | 1.48× |
| 100,000 | 0.935 | 106.96M | 0.886 | 112.91M | 0.978 | 1.05× | 1.10× |
| 1,000,000 | 9.814 | 101.89M | 10.203 | 98.01M | 10.655 | 1.09× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.161 | 1.91× |
| 1 | 5 | 0.287 | 0.551 | 1.92× |
| 1 | 10 | 0.506 | 0.959 | 1.90× |
| 10 | 1 | 0.050 | 0.098 | 1.98× |
| 10 | 5 | 0.227 | 0.443 | 1.95× |
| 10 | 10 | 0.546 | 1.045 | 1.91× |
| 100 | 1 | 0.065 | 0.104 | 1.60× |
| 100 | 5 | 0.243 | 0.454 | 1.87× |
| 100 | 10 | 0.498 | 1.120 | 2.25× |
| 1,000 | 1 | 0.068 | 0.107 | 1.57× |
| 1,000 | 5 | 0.249 | 0.492 | 1.98× |
| 1,000 | 10 | 0.510 | 1.170 | 2.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
