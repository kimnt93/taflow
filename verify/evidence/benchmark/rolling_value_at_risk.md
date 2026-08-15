# RollingValueAtRisk benchmark (`ValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.131 | 7.63M | 0.133 | 7.53M | 0.348 | 2.65× | 2.62× |
| 10,000 | 1.339 | 7.47M | 1.340 | 7.46M | 1.789 | 1.34× | 1.34× |
| 100,000 | 13.503 | 7.41M | 13.213 | 7.57M | 16.798 | 1.24× | 1.27× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.321 | 4.07× |
| 1 | 5 | 0.296 | 1.193 | 4.03× |
| 1 | 10 | 0.413 | 2.672 | 6.47× |
| 10 | 1 | 0.044 | 0.238 | 5.34× |
| 10 | 5 | 0.202 | 1.153 | 5.71× |
| 10 | 10 | 0.409 | 2.549 | 6.24× |
| 100 | 1 | 0.057 | 0.251 | 4.41× |
| 100 | 5 | 0.207 | 1.437 | 6.95× |
| 100 | 10 | 0.451 | 2.776 | 6.15× |
| 1,000 | 1 | 0.189 | 0.407 | 2.16× |
| 1,000 | 5 | 0.351 | 2.277 | 6.49× |
| 1,000 | 10 | 0.556 | 4.339 | 7.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
