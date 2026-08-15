# Aroon benchmark (`AROON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 96.11M | 0.009 | 115.24M | 0.043 | 4.14× | 4.96× |
| 10,000 | 0.119 | 84.12M | 0.110 | 90.60M | 0.142 | 1.19× | 1.29× |
| 100,000 | 1.927 | 51.91M | 1.151 | 86.91M | 1.139 | 0.59× | 0.99× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.160 | 0.169 | 1.05× |
| 1 | 5 | 0.266 | 0.526 | 1.97× |
| 1 | 10 | 0.383 | 1.010 | 2.64× |
| 10 | 1 | 0.044 | 0.098 | 2.23× |
| 10 | 5 | 0.183 | 0.459 | 2.52× |
| 10 | 10 | 0.439 | 0.997 | 2.27× |
| 100 | 1 | 0.048 | 0.102 | 2.15× |
| 100 | 5 | 0.188 | 0.470 | 2.50× |
| 100 | 10 | 0.397 | 1.028 | 2.59× |
| 1,000 | 1 | 0.058 | 0.112 | 1.95× |
| 1,000 | 5 | 0.217 | 0.513 | 2.36× |
| 1,000 | 10 | 0.419 | 1.082 | 2.58× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
