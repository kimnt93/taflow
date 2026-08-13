# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.140 | 7.13M | 0.092 | 10.82M | 0.035 | 0.25× | 0.38× |
| 10,000 | 0.796 | 12.57M | 0.788 | 12.70M | 0.092 | 0.12× | 0.12× |
| 100,000 | 7.601 | 13.16M | 7.851 | 12.74M | 0.676 | 0.09× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.102 | 0.132 | 1.30× |
| 1 | 5 | 0.456 | 0.482 | 1.06× |
| 1 | 10 | 0.598 | 0.933 | 1.56× |
| 10 | 1 | 0.064 | 0.097 | 1.50× |
| 10 | 5 | 0.288 | 0.434 | 1.51× |
| 10 | 10 | 0.629 | 0.962 | 1.53× |
| 100 | 1 | 0.079 | 0.098 | 1.24× |
| 100 | 5 | 0.313 | 0.449 | 1.43× |
| 100 | 10 | 0.625 | 0.951 | 1.52× |
| 1,000 | 1 | 0.155 | 0.100 | 0.65× |
| 1,000 | 5 | 0.364 | 0.495 | 1.36× |
| 1,000 | 10 | 0.666 | 1.012 | 1.52× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
