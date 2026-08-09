# MathFloor benchmark (`FLOOR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 168.70M | 0.005 | 210.27M | 0.029 | 4.81× | 6.00× |
| 10,000 | 0.031 | 319.14M | 0.028 | 355.98M | 0.039 | 1.25× | 1.40× |
| 100,000 | 0.311 | 321.59M | 0.248 | 403.91M | 0.154 | 0.49× | 0.62× |
| 1,000,000 | 3.700 | 270.30M | 3.157 | 316.81M | 1.368 | 0.37× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.198 | 2.04× |
| 1 | 5 | 0.351 | 0.521 | 1.48× |
| 1 | 10 | 0.476 | 0.898 | 1.89× |
| 10 | 1 | 0.047 | 0.084 | 1.79× |
| 10 | 5 | 0.227 | 0.406 | 1.79× |
| 10 | 10 | 0.472 | 0.900 | 1.91× |
| 100 | 1 | 0.049 | 0.097 | 1.99× |
| 100 | 5 | 0.216 | 0.426 | 1.97× |
| 100 | 10 | 0.474 | 0.880 | 1.85× |
| 1,000 | 1 | 0.052 | 0.086 | 1.67× |
| 1,000 | 5 | 0.231 | 0.420 | 1.82× |
| 1,000 | 10 | 0.497 | 0.903 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
