# MathCeil benchmark (`CEIL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.81M | 0.004 | 235.21M | 0.029 | 5.19× | 6.91× |
| 10,000 | 0.026 | 389.95M | 0.024 | 421.95M | 0.042 | 1.62× | 1.75× |
| 100,000 | 0.227 | 440.01M | 0.210 | 477.05M | 0.167 | 0.73× | 0.80× |
| 1,000,000 | 2.701 | 370.27M | 2.219 | 450.74M | 1.433 | 0.53× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.129 | 1.50× |
| 1 | 5 | 0.347 | 0.432 | 1.25× |
| 1 | 10 | 0.445 | 0.888 | 2.00× |
| 10 | 1 | 0.050 | 0.088 | 1.76× |
| 10 | 5 | 0.224 | 0.406 | 1.81× |
| 10 | 10 | 0.466 | 0.907 | 1.94× |
| 100 | 1 | 0.048 | 0.086 | 1.78× |
| 100 | 5 | 0.215 | 0.424 | 1.97× |
| 100 | 10 | 0.483 | 0.883 | 1.83× |
| 1,000 | 1 | 0.047 | 0.091 | 1.93× |
| 1,000 | 5 | 0.230 | 0.432 | 1.88× |
| 1,000 | 10 | 0.488 | 0.916 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
