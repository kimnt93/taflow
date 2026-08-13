# RollingSkew benchmark (`Skewness` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.079 | 12.73M | 0.069 | 14.47M | 0.190 | 2.42× | 2.75× |
| 10,000 | 0.633 | 15.81M | 0.619 | 16.15M | 0.744 | 1.18× | 1.20× |
| 100,000 | 6.228 | 16.06M | 6.452 | 15.50M | 6.420 | 1.03× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.109 | 0.231 | 2.11× |
| 1 | 5 | 0.349 | 1.027 | 2.94× |
| 1 | 10 | 0.612 | 2.241 | 3.66× |
| 10 | 1 | 0.067 | 0.214 | 3.20× |
| 10 | 5 | 0.289 | 1.203 | 4.16× |
| 10 | 10 | 0.605 | 2.234 | 3.69× |
| 100 | 1 | 0.077 | 0.214 | 2.80× |
| 100 | 5 | 0.310 | 1.229 | 3.97× |
| 100 | 10 | 0.656 | 2.272 | 3.47× |
| 1,000 | 1 | 0.133 | 0.272 | 2.04× |
| 1,000 | 5 | 0.316 | 1.511 | 4.79× |
| 1,000 | 10 | 0.666 | 2.832 | 4.25× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
